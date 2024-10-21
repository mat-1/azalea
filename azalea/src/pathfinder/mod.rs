//! A pathfinding plugin to make bots able to traverse the world.
//!
//! Much of this code is based on [Baritone](https://github.com/cabaletta/baritone).

pub mod astar;
pub mod block_box;
pub mod costs;
mod debug;
pub mod extras;
pub mod goals;
pub mod mining;
pub mod moves;
pub mod simulation;
pub mod world;

use crate::auto_tool::StartMiningBlockWithAutoToolEvent;
use crate::bot::{JumpEvent, LookAtEvent};
use crate::pathfinder::astar::a_star;
use crate::WalkDirection;

use crate::app::{App, Plugin};
use crate::ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    query::{With, Without},
    system::{Commands, Query, Res},
};
use crate::pathfinder::moves::PathfinderCtx;
use crate::pathfinder::world::CachedWorld;
use azalea_client::inventory::{InventoryComponent, InventorySet, SetSelectedHotbarSlotEvent};
use azalea_client::mining::Mining;
use azalea_client::movement::MoveEventsSet;
use azalea_client::{InstanceHolder, StartSprintEvent, StartWalkEvent};
use azalea_core::position::BlockPos;
use azalea_core::tick::GameTick;
use azalea_entity::metadata::Player;
use azalea_entity::LocalEntity;
use azalea_entity::{Physics, Position};
use azalea_physics::PhysicsSet;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{PreUpdate, Update};
use bevy_ecs::prelude::Event;
use bevy_ecs::query::Changed;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use std::collections::VecDeque;
use std::sync::atomic::{self, AtomicUsize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, trace, warn};

use self::debug::debug_render_path_with_particles;
pub use self::debug::PathfinderDebugParticles;
use self::goals::Goal;
use self::mining::MiningCache;
use self::moves::{ExecuteCtx, IsReachedCtx, SuccessorsFn};

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoEvent>()
            .add_event::<PathFoundEvent>()
            .add_event::<StopPathfindingEvent>()
            .add_systems(
                // putting systems in the GameTick schedule makes them run every Minecraft tick
                // (every 50 milliseconds).
                GameTick,
                (
                    timeout_movement,
                    check_node_reached,
                    tick_execute_path,
                    check_for_path_obstruction,
                    debug_render_path_with_particles,
                    recalculate_near_end_of_path,
                    recalculate_if_has_goal_but_no_path,
                )
                    .chain()
                    .after(PhysicsSet)
                    .after(azalea_client::movement::send_position),
            )
            .add_systems(PreUpdate, add_default_pathfinder)
            .add_systems(
                Update,
                (
                    goto_listener,
                    handle_tasks,
                    path_found_listener,
                    stop_pathfinding_on_instance_change,
                    handle_stop_pathfinding_event,
                )
                    .chain()
                    .before(MoveEventsSet)
                    .before(InventorySet),
            )
            .add_plugins(crate::pathfinder::extras::PathfinderExtrasPlugin);
    }
}

pub trait PathfinderClientExt {
    fn goto(&self, goal: impl Goal + Send + Sync + 'static);
    fn goto_without_mining(&self, goal: impl Goal + Send + Sync + 'static);
    fn stop_pathfinding(&self);
}

/// A component that makes this client able to pathfind.
#[derive(Component, Default, Clone)]
pub struct Pathfinder {
    /// The goal that the pathfinder is currently trying to reach. If this is
    /// None, that means we're not pathfinding.
    pub goal: Option<Arc<dyn Goal>>,
    pub successors_fn: Option<SuccessorsFn>,
    pub is_calculating: bool,
    pub allow_mining: bool,

    pub goto_id: Arc<AtomicUsize>,
}

/// A component that's present on clients that are actively following a
/// pathfinder path.
#[derive(Component)]
pub struct ExecutingPath {
    pub path: VecDeque<astar::Movement<BlockPos, moves::MoveData>>,
    pub queued_path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub last_reached_node: BlockPos,
    pub last_node_reached_at: Instant,
    pub is_path_partial: bool,
}

#[derive(Event)]
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Arc<dyn Goal>,
    /// The function that's used for checking what moves are possible. Usually
    /// `pathfinder::moves::default_move`
    pub successors_fn: SuccessorsFn,

    /// Whether the bot is allowed to break blocks while pathfinding.
    pub allow_mining: bool,
}
#[derive(Event, Clone)]
pub struct PathFoundEvent {
    pub entity: Entity,
    pub start: BlockPos,
    pub path: Option<VecDeque<astar::Movement<BlockPos, moves::MoveData>>>,
    pub is_partial: bool,
    pub successors_fn: SuccessorsFn,
    pub allow_mining: bool,
}

#[allow(clippy::type_complexity)]
fn add_default_pathfinder(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Pathfinder>, With<LocalEntity>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(Pathfinder::default());
    }
}

impl PathfinderClientExt for azalea_client::Client {
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::goals::BlockPosGoal};
    /// # fn example(bot: &Client) {
    /// bot.goto(BlockPosGoal(BlockPos::new(0, 70, 0)));
    /// # }
    /// ```
    fn goto(&self, goal: impl Goal + Send + Sync + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
            successors_fn: moves::default_move,
            allow_mining: true,
        });
    }

    /// Same as [`goto`](Self::goto). but the bot won't break any blocks while
    /// executing the path.
    fn goto_without_mining(&self, goal: impl Goal + Send + Sync + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
            successors_fn: moves::default_move,
            allow_mining: false,
        });
    }

    fn stop_pathfinding(&self) {
        self.ecs.lock().send_event(StopPathfindingEvent {
            entity: self.entity,
            force: false,
        });
    }
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

pub fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(
        &mut Pathfinder,
        Option<&ExecutingPath>,
        &Position,
        &InstanceName,
        &InventoryComponent,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.read() {
        let Ok((mut pathfinder, executing_path, position, instance_name, inventory)) =
            query.get_mut(event.entity)
        else {
            continue;
        };

        if event.goal.success(BlockPos::from(position)) {
            // we're already at the goal, nothing to do
            pathfinder.goal = None;
            pathfinder.successors_fn = None;
            pathfinder.is_calculating = false;
            continue;
        }

        // we store the goal so it can be recalculated later if necessary
        pathfinder.goal = Some(event.goal.clone());
        pathfinder.successors_fn = Some(event.successors_fn);
        pathfinder.is_calculating = true;
        pathfinder.allow_mining = event.allow_mining;

        let start = if let Some(executing_path) = executing_path
            && let Some(final_node) = executing_path.path.back()
        {
            // if we're currently pathfinding and got a goto event, start a little ahead
            executing_path.path.get(20).unwrap_or(final_node).target
        } else {
            BlockPos::from(position)
        };
        info!(
            "got goto, starting from {start:?} (currently at {:?})",
            BlockPos::from(position)
        );

        let successors_fn: moves::SuccessorsFn = event.successors_fn;

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        let goal = event.goal.clone();
        let entity = event.entity;

        let goto_id_atomic = pathfinder.goto_id.clone();
        let goto_id = goto_id_atomic.fetch_add(1, atomic::Ordering::Relaxed) + 1;

        let allow_mining = event.allow_mining;
        let mining_cache = MiningCache::new(if allow_mining {
            Some(inventory.inventory_menu.clone())
        } else {
            None
        });

        let task = thread_pool.spawn(async move {
            debug!("start: {start:?}");

            let cached_world = CachedWorld::new(world_lock);
            let successors = |pos: BlockPos| {
                call_successors_fn(&cached_world, &mining_cache, successors_fn, pos)
            };

            let mut attempt_number = 0;

            let mut path;
            let mut is_partial: bool;

            'calculate: loop {
                let start_time = std::time::Instant::now();
                let astar::Path { movements, partial } = a_star(
                    start,
                    |n| goal.heuristic(n),
                    successors,
                    |n| goal.success(n),
                    Duration::from_secs(if attempt_number == 0 { 1 } else { 5 }),
                );
                let end_time = std::time::Instant::now();
                debug!("partial: {partial:?}");
                let duration = end_time - start_time;
                if partial {
                    if movements.is_empty() {
                        info!("Pathfinder took {duration:?} (empty path)");
                    } else {
                        info!("Pathfinder took {duration:?} (incomplete path)");
                    }
                    // wait a bit so it's not a busy loop
                    std::thread::sleep(Duration::from_millis(100));
                } else {
                    info!("Pathfinder took {duration:?}");
                }

                debug!("Path:");
                for movement in &movements {
                    debug!("  {:?}", movement.target);
                }

                path = movements.into_iter().collect::<VecDeque<_>>();
                is_partial = partial;

                let goto_id_now = goto_id_atomic.load(atomic::Ordering::Relaxed);
                if goto_id != goto_id_now {
                    // we must've done another goto while calculating this path, so throw it away
                    warn!("finished calculating a path, but it's outdated");
                    return None;
                }

                if path.is_empty() && partial {
                    if attempt_number == 0 {
                        debug!("this path is empty, retrying with a higher timeout");
                        attempt_number += 1;
                        continue 'calculate;
                    } else {
                        debug!("this path is empty, giving up");
                        break 'calculate;
                    }
                }
                break;
            }

            Some(PathFoundEvent {
                entity,
                start,
                path: Some(path),
                is_partial,
                successors_fn,
                allow_mining,
            })
        });

        commands.entity(event.entity).insert(ComputePath(task));
    }
}

// poll the tasks and send the PathFoundEvent if they're done
fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputePath)>,
    mut path_found_events: EventWriter<PathFoundEvent>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(optional_path_found_event) = future::block_on(future::poll_once(&mut task.0)) {
            if let Some(path_found_event) = optional_path_found_event {
                path_found_events.send(path_found_event);
            }

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<ComputePath>();
        }
    }
}

// set the path for the target entity when we get the PathFoundEvent
fn path_found_listener(
    mut events: EventReader<PathFoundEvent>,
    mut query: Query<(
        &mut Pathfinder,
        Option<&mut ExecutingPath>,
        &InstanceName,
        &InventoryComponent,
    )>,
    instance_container: Res<InstanceContainer>,
    mut commands: Commands,
) {
    for event in events.read() {
        let (mut pathfinder, executing_path, instance_name, inventory) = query
            .get_mut(event.entity)
            .expect("Path found for an entity that doesn't have a pathfinder");
        if let Some(path) = &event.path {
            if let Some(mut executing_path) = executing_path {
                let mut new_path = VecDeque::new();

                // combine the old and new paths if the first node of the new path is a
                // successor of the last node of the old path
                if let Some(last_node_of_current_path) = executing_path.path.back() {
                    let world_lock = instance_container
                        .get(instance_name)
                        .expect("Entity tried to pathfind but the entity isn't in a valid world");
                    let successors_fn: moves::SuccessorsFn = event.successors_fn;
                    let cached_world = CachedWorld::new(world_lock);
                    let mining_cache = MiningCache::new(if event.allow_mining {
                        Some(inventory.inventory_menu.clone())
                    } else {
                        None
                    });
                    let successors = |pos: BlockPos| {
                        call_successors_fn(&cached_world, &mining_cache, successors_fn, pos)
                    };

                    if let Some(first_node_of_new_path) = path.front() {
                        if successors(last_node_of_current_path.target)
                            .iter()
                            .any(|edge| edge.movement.target == first_node_of_new_path.target)
                        {
                            debug!("combining old and new paths");
                            debug!(
                                "old path: {:?}",
                                executing_path.path.iter().collect::<Vec<_>>()
                            );
                            debug!("new path: {:?}", path.iter().take(10).collect::<Vec<_>>());
                            new_path.extend(executing_path.path.iter().cloned());
                        }
                    } else {
                        new_path.extend(executing_path.path.iter().cloned());
                    }
                }

                new_path.extend(path.to_owned());

                debug!(
                    "set queued path to {:?}",
                    new_path.iter().take(10).collect::<Vec<_>>()
                );
                executing_path.queued_path = Some(new_path);
                executing_path.is_path_partial = event.is_partial;
            } else if path.is_empty() {
                debug!("calculated path is empty, so didn't add ExecutingPath");
            } else {
                commands.entity(event.entity).insert(ExecutingPath {
                    path: path.to_owned(),
                    queued_path: None,
                    last_reached_node: event.start,
                    last_node_reached_at: Instant::now(),
                    is_path_partial: event.is_partial,
                });
                debug!("set path to {:?}", path.iter().take(10).collect::<Vec<_>>());
                debug!("partial: {}", event.is_partial);
            }
        } else {
            error!("No path found");
            if let Some(mut executing_path) = executing_path {
                // set the queued path so we don't stop in the middle of a move
                executing_path.queued_path = Some(VecDeque::new());
            } else {
                // wasn't executing a path, don't need to do anything
            }
        }
        pathfinder.is_calculating = false;
    }
}

fn timeout_movement(
    mut query: Query<(&Pathfinder, &mut ExecutingPath, &Position, Option<&Mining>)>,
) {
    for (pathfinder, mut executing_path, position, mining) in &mut query {
        // don't timeout if we're mining
        if let Some(mining) = mining {
            // also make sure we're close enough to the block that's being mined
            if mining.pos.distance_squared_to(&BlockPos::from(position)) < 6_i32.pow(2) {
                // also reset the last_node_reached_at so we don't timeout after we finish
                // mining
                executing_path.last_node_reached_at = Instant::now();
                continue;
            }
        }

        if executing_path.last_node_reached_at.elapsed() > Duration::from_secs(2)
            && !pathfinder.is_calculating
            && !executing_path.path.is_empty()
        {
            warn!("pathfinder timeout");
            // the path wasn't being followed anyways, so clearing it is fine
            executing_path.path.clear();
            executing_path.queued_path = None;
            executing_path.last_reached_node = BlockPos::from(position);
            // invalidate whatever calculation we were just doing, if any
            pathfinder.goto_id.fetch_add(1, atomic::Ordering::Relaxed);
            // set partial to true to make sure that a recalculation will happen
            executing_path.is_path_partial = true;

            // the path will get recalculated automatically because the path is
            // empty
        }
    }
}

fn check_node_reached(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &mut ExecutingPath,
        &Position,
        &Physics,
    )>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut commands: Commands,
) {
    for (entity, mut pathfinder, mut executing_path, position, physics) in &mut query {
        'skip: loop {
            // we check if the goal was reached *before* actually executing the movement so
            // we don't unnecessarily execute a movement when it wasn't necessary

            // see if we already reached any future nodes and can skip ahead
            for (i, movement) in executing_path
                .path
                .clone()
                .into_iter()
                .enumerate()
                .take(20)
                .rev()
            {
                let is_reached_ctx = IsReachedCtx {
                    target: movement.target,
                    start: executing_path.last_reached_node,
                    position: **position,
                    physics,
                };
                let extra_strict_if_last = if i == executing_path.path.len() - 1 {
                    let x_difference_from_center = position.x - (movement.target.x as f64 + 0.5);
                    let z_difference_from_center = position.z - (movement.target.z as f64 + 0.5);
                    // this is to make sure we don't fall off immediately after finishing the path
                    physics.on_ground
                    && BlockPos::from(position) == movement.target
                    // adding the delta like this isn't a perfect solution but it helps to make
                    // sure we don't keep going if our delta is high
                    && (x_difference_from_center + physics.velocity.x).abs() < 0.2
                    && (z_difference_from_center + physics.velocity.z).abs() < 0.2
                } else {
                    true
                };
                if (movement.data.is_reached)(is_reached_ctx) && extra_strict_if_last {
                    executing_path.path = executing_path.path.split_off(i + 1);
                    executing_path.last_reached_node = movement.target;
                    executing_path.last_node_reached_at = Instant::now();

                    if let Some(new_path) = executing_path.queued_path.take() {
                        debug!(
                            "swapped path to {:?}",
                            new_path.iter().take(10).collect::<Vec<_>>()
                        );
                        executing_path.path = new_path;

                        if executing_path.path.is_empty() {
                            info!("the path we just swapped to was empty, so reached end of path");
                            walk_events.send(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                            commands.entity(entity).remove::<ExecutingPath>();
                            break;
                        }

                        // run the function again since we just swapped
                        continue 'skip;
                    }

                    if executing_path.path.is_empty() {
                        debug!("pathfinder path is now empty");
                        walk_events.send(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                        commands.entity(entity).remove::<ExecutingPath>();
                        if let Some(goal) = pathfinder.goal.clone() {
                            if goal.success(movement.target) {
                                info!("goal was reached!");
                                pathfinder.goal = None;
                                pathfinder.successors_fn = None;
                            }
                        }
                    }

                    break;
                }
            }
            break;
        }
    }
}

fn check_for_path_obstruction(
    mut query: Query<(
        &Pathfinder,
        &mut ExecutingPath,
        &InstanceName,
        &InventoryComponent,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (pathfinder, mut executing_path, instance_name, inventory) in &mut query {
        let Some(successors_fn) = pathfinder.successors_fn else {
            continue;
        };

        let world_lock = instance_container
            .get(instance_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");

        // obstruction check (the path we're executing isn't possible anymore)
        let cached_world = CachedWorld::new(world_lock);
        let mining_cache = MiningCache::new(if pathfinder.allow_mining {
            Some(inventory.inventory_menu.clone())
        } else {
            None
        });
        let successors =
            |pos: BlockPos| call_successors_fn(&cached_world, &mining_cache, successors_fn, pos);

        if let Some(obstructed_index) = check_path_obstructed(
            executing_path.last_reached_node,
            &executing_path.path,
            successors,
        ) {
            warn!(
                "path obstructed at index {obstructed_index} (starting at {:?}, path: {:?})",
                executing_path.last_reached_node, executing_path.path
            );
            executing_path.path.truncate(obstructed_index);
            executing_path.is_path_partial = true;
        }
    }
}

pub fn recalculate_near_end_of_path(
    mut query: Query<(Entity, &mut Pathfinder, &mut ExecutingPath)>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut goto_events: EventWriter<GotoEvent>,
    mut commands: Commands,
) {
    for (entity, mut pathfinder, mut executing_path) in &mut query {
        let Some(successors_fn) = pathfinder.successors_fn else {
            continue;
        };

        // start recalculating if the path ends soon
        if (executing_path.path.len() == 20 || executing_path.path.len() < 5)
            && !pathfinder.is_calculating
            && executing_path.is_path_partial
        {
            if let Some(goal) = pathfinder.goal.as_ref().cloned() {
                debug!("Recalculating path because it ends soon");
                debug!(
                    "recalculate_near_end_of_path executing_path.is_path_partial: {}",
                    executing_path.is_path_partial
                );
                goto_events.send(GotoEvent {
                    entity,
                    goal,
                    successors_fn,
                    allow_mining: pathfinder.allow_mining,
                });
                pathfinder.is_calculating = true;

                if executing_path.path.is_empty() {
                    if let Some(new_path) = executing_path.queued_path.take() {
                        executing_path.path = new_path;
                        if executing_path.path.is_empty() {
                            info!("the path we just swapped to was empty, so reached end of path");
                            walk_events.send(StartWalkEvent {
                                entity,
                                direction: WalkDirection::None,
                            });
                            commands.entity(entity).remove::<ExecutingPath>();
                            break;
                        }
                    } else {
                        walk_events.send(StartWalkEvent {
                            entity,
                            direction: WalkDirection::None,
                        });
                        commands.entity(entity).remove::<ExecutingPath>();
                    }
                }
            } else if executing_path.path.is_empty() {
                // idk when this can happen but stop moving just in case
                walk_events.send(StartWalkEvent {
                    entity,
                    direction: WalkDirection::None,
                });
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn tick_execute_path(
    mut query: Query<(
        Entity,
        &mut ExecutingPath,
        &Position,
        &Physics,
        Option<&Mining>,
        &InstanceHolder,
        &InventoryComponent,
    )>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
    mut start_mining_events: EventWriter<StartMiningBlockWithAutoToolEvent>,
    mut set_selected_hotbar_slot_events: EventWriter<SetSelectedHotbarSlotEvent>,
) {
    for (entity, executing_path, position, physics, mining, instance_holder, inventory_component) in
        &mut query
    {
        if let Some(movement) = executing_path.path.front() {
            let ctx = ExecuteCtx {
                entity,
                target: movement.target,
                position: **position,
                start: executing_path.last_reached_node,
                physics,
                is_currently_mining: mining.is_some(),
                instance: instance_holder.instance.clone(),
                menu: inventory_component.inventory_menu.clone(),

                look_at_events: &mut look_at_events,
                sprint_events: &mut sprint_events,
                walk_events: &mut walk_events,
                jump_events: &mut jump_events,
                start_mining_events: &mut start_mining_events,
                set_selected_hotbar_slot_events: &mut set_selected_hotbar_slot_events,
            };
            trace!("executing move");
            (movement.data.execute)(ctx);
        }
    }
}

fn recalculate_if_has_goal_but_no_path(
    mut query: Query<(Entity, &mut Pathfinder), Without<ExecutingPath>>,
    mut goto_events: EventWriter<GotoEvent>,
) {
    for (entity, mut pathfinder) in &mut query {
        if pathfinder.goal.is_some() && !pathfinder.is_calculating {
            if let Some(goal) = pathfinder.goal.as_ref().cloned() {
                debug!("Recalculating path because it has a goal but no ExecutingPath");
                goto_events.send(GotoEvent {
                    entity,
                    goal,
                    successors_fn: pathfinder.successors_fn.unwrap(),
                    allow_mining: pathfinder.allow_mining,
                });
                pathfinder.is_calculating = true;
            }
        }
    }
}

#[derive(Event)]
pub struct StopPathfindingEvent {
    pub entity: Entity,
    /// Stop moving immediately. This may cause the bot to fall if it was in the
    /// middle of parkouring.
    ///
    /// If this is false, it'll stop moving after the current movement is done.
    pub force: bool,
}

pub fn handle_stop_pathfinding_event(
    mut events: EventReader<StopPathfindingEvent>,
    mut query: Query<(&mut Pathfinder, &mut ExecutingPath)>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        // stop computing any path that's being computed
        commands.entity(event.entity).remove::<ComputePath>();

        let Ok((mut pathfinder, mut executing_path)) = query.get_mut(event.entity) else {
            continue;
        };
        pathfinder.goal = None;
        if event.force {
            executing_path.path.clear();
            executing_path.queued_path = None;
        } else {
            // switch to an empty path as soon as it can
            executing_path.queued_path = Some(VecDeque::new());
            // make sure it doesn't recalculate
            executing_path.is_path_partial = false;
        }

        if executing_path.path.is_empty() {
            walk_events.send(StartWalkEvent {
                entity: event.entity,
                direction: WalkDirection::None,
            });
            commands.entity(event.entity).remove::<ExecutingPath>();
        }
    }
}

pub fn stop_pathfinding_on_instance_change(
    mut query: Query<(Entity, &mut ExecutingPath), Changed<InstanceName>>,
    mut stop_pathfinding_events: EventWriter<StopPathfindingEvent>,
) {
    for (entity, mut executing_path) in &mut query {
        if !executing_path.path.is_empty() {
            debug!("instance changed, clearing path");
            executing_path.path.clear();
            stop_pathfinding_events.send(StopPathfindingEvent {
                entity,
                force: true,
            });
        }
    }
}

/// Checks whether the path has been obstructed, and returns Some(index) if it
/// has been. The index is of the first obstructed node.
fn check_path_obstructed<SuccessorsFn>(
    mut current_position: BlockPos,
    path: &VecDeque<astar::Movement<BlockPos, moves::MoveData>>,
    successors_fn: SuccessorsFn,
) -> Option<usize>
where
    SuccessorsFn: Fn(BlockPos) -> Vec<astar::Edge<BlockPos, moves::MoveData>>,
{
    for (i, movement) in path.iter().enumerate() {
        let mut found_obstruction = false;
        for edge in successors_fn(current_position) {
            if edge.movement.target == movement.target {
                current_position = movement.target;
                found_obstruction = false;
                break;
            } else {
                found_obstruction = true;
            }
        }
        if found_obstruction {
            return Some(i);
        }
    }

    None
}

pub fn call_successors_fn(
    cached_world: &CachedWorld,
    mining_cache: &MiningCache,
    successors_fn: SuccessorsFn,
    pos: BlockPos,
) -> Vec<astar::Edge<BlockPos, moves::MoveData>> {
    let mut edges = Vec::with_capacity(16);
    let mut ctx = PathfinderCtx {
        edges: &mut edges,
        world: cached_world,
        mining_cache,
    };
    successors_fn(&mut ctx, pos);
    edges
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        sync::Arc,
        time::{Duration, Instant},
    };

    use azalea_core::position::{BlockPos, ChunkPos, Vec3};
    use azalea_world::{Chunk, ChunkStorage, PartialChunkStorage};

    use super::{
        goals::BlockPosGoal,
        moves,
        simulation::{SimulatedPlayerBundle, Simulation},
        GotoEvent,
    };

    fn setup_blockposgoal_simulation(
        partial_chunks: &mut PartialChunkStorage,
        start_pos: BlockPos,
        end_pos: BlockPos,
        solid_blocks: Vec<BlockPos>,
    ) -> Simulation {
        let mut simulation = setup_simulation_world(partial_chunks, start_pos, solid_blocks);

        // you can uncomment this while debugging tests to get trace logs
        // simulation.app.add_plugins(bevy_log::LogPlugin {
        //     level: bevy_log::Level::TRACE,
        //     filter: "".to_string(),
        // });

        simulation.app.world.send_event(GotoEvent {
            entity: simulation.entity,
            goal: Arc::new(BlockPosGoal(end_pos)),
            successors_fn: moves::default_move,
            allow_mining: false,
        });
        simulation
    }

    fn setup_simulation_world(
        partial_chunks: &mut PartialChunkStorage,
        start_pos: BlockPos,
        solid_blocks: Vec<BlockPos>,
    ) -> Simulation {
        let mut chunk_positions = HashSet::new();
        for block_pos in &solid_blocks {
            chunk_positions.insert(ChunkPos::from(block_pos));
        }

        let mut chunks = ChunkStorage::default();
        for chunk_pos in chunk_positions {
            partial_chunks.set(&chunk_pos, Some(Chunk::default()), &mut chunks);
        }
        for block_pos in solid_blocks {
            chunks.set_block_state(&block_pos, azalea_registry::Block::Stone.into());
        }
        let player = SimulatedPlayerBundle::new(Vec3::new(
            start_pos.x as f64 + 0.5,
            start_pos.y as f64,
            start_pos.z as f64 + 0.5,
        ));
        Simulation::new(chunks, player)
    }

    pub fn assert_simulation_reaches(simulation: &mut Simulation, ticks: usize, end_pos: BlockPos) {
        wait_until_bot_starts_moving(simulation);
        for _ in 0..ticks {
            simulation.tick();
        }
        assert_eq!(BlockPos::from(simulation.position()), end_pos);
    }

    pub fn wait_until_bot_starts_moving(simulation: &mut Simulation) {
        let start_pos = simulation.position();
        let start_time = Instant::now();
        while simulation.position() == start_pos
            && !simulation.is_mining()
            && start_time.elapsed() < Duration::from_millis(500)
        {
            simulation.tick();
            std::thread::yield_now();
        }
    }

    #[test]
    fn test_simple_forward() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 71, 1),
            vec![BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 1)],
        );
        assert_simulation_reaches(&mut simulation, 20, BlockPos::new(0, 71, 1));
    }

    #[test]
    fn test_double_diagonal_with_walls() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(2, 71, 2),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(1, 70, 1),
                BlockPos::new(2, 70, 2),
                BlockPos::new(1, 72, 0),
                BlockPos::new(2, 72, 1),
            ],
        );
        assert_simulation_reaches(&mut simulation, 30, BlockPos::new(2, 71, 2));
    }

    #[test]
    fn test_jump_with_sideways_momentum() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 3),
            BlockPos::new(5, 76, 0),
            vec![
                BlockPos::new(0, 70, 3),
                BlockPos::new(0, 70, 2),
                BlockPos::new(0, 70, 1),
                BlockPos::new(0, 70, 0),
                BlockPos::new(1, 71, 0),
                BlockPos::new(2, 72, 0),
                BlockPos::new(3, 73, 0),
                BlockPos::new(4, 74, 0),
                BlockPos::new(5, 75, 0),
            ],
        );
        assert_simulation_reaches(&mut simulation, 120, BlockPos::new(5, 76, 0));
    }

    #[test]
    fn test_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 71, 3),
            vec![BlockPos::new(0, 70, 0), BlockPos::new(0, 70, 3)],
        );
        assert_simulation_reaches(&mut simulation, 40, BlockPos::new(0, 71, 3));
    }

    #[test]
    fn test_descend_and_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(3, 67, 4),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 69, 1),
                BlockPos::new(0, 68, 2),
                BlockPos::new(0, 67, 3),
                BlockPos::new(0, 66, 4),
                BlockPos::new(3, 66, 4),
            ],
        );
        assert_simulation_reaches(&mut simulation, 100, BlockPos::new(3, 67, 4));
    }

    #[test]
    fn test_small_descend_and_parkour_2_block_gap() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 70, 5),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 1),
                BlockPos::new(0, 69, 2),
                BlockPos::new(0, 69, 5),
            ],
        );
        assert_simulation_reaches(&mut simulation, 40, BlockPos::new(0, 70, 5));
    }

    #[test]
    fn test_quickly_descend() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(0, 68, 3),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 69, 1),
                BlockPos::new(0, 68, 2),
                BlockPos::new(0, 67, 3),
            ],
        );
        assert_simulation_reaches(&mut simulation, 60, BlockPos::new(0, 68, 3));
    }

    #[test]
    fn test_2_gap_ascend_thrice() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(3, 74, 0),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 71, 3),
                BlockPos::new(3, 72, 3),
                BlockPos::new(3, 73, 0),
            ],
        );
        assert_simulation_reaches(&mut simulation, 60, BlockPos::new(3, 74, 0));
    }

    #[test]
    fn test_consecutive_3_gap_parkour() {
        let mut partial_chunks = PartialChunkStorage::default();
        let mut simulation = setup_blockposgoal_simulation(
            &mut partial_chunks,
            BlockPos::new(0, 71, 0),
            BlockPos::new(4, 71, 12),
            vec![
                BlockPos::new(0, 70, 0),
                BlockPos::new(0, 70, 4),
                BlockPos::new(0, 70, 8),
                BlockPos::new(0, 70, 12),
                BlockPos::new(4, 70, 12),
            ],
        );
        assert_simulation_reaches(&mut simulation, 80, BlockPos::new(4, 71, 12));
    }
}
