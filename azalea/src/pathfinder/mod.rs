mod astar;
mod execute;
pub mod goals;
mod moves;

use crate::pathfinder::astar::a_star;

use crate::app::{App, Plugin};
use crate::ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    query::{With, Without},
    system::{Commands, Query, Res},
};
use astar::Edge;
use azalea_core::{BlockPos, CardinalDirection};
use azalea_entity::metadata::Player;
use azalea_entity::Local;
use azalea_entity::{Physics, Position};
use azalea_physics::PhysicsSet;
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{FixedUpdate, Update};
use bevy_ecs::prelude::Event;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use log::{debug, error};
use std::collections::VecDeque;
use std::sync::Arc;

use self::execute::tick_execute_path;

#[derive(Clone, Default)]
pub struct PathfinderPlugin;
impl Plugin for PathfinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GotoEvent>()
            .add_event::<PathFoundEvent>()
            .add_systems(
                FixedUpdate,
                // putting systems in the FixedUpdate schedule makes them run every Minecraft tick
                // (every 50 milliseconds).
                tick_execute_path.before(PhysicsSet),
            )
            .add_systems(
                Update,
                (
                    goto_listener,
                    add_default_pathfinder,
                    (handle_tasks, path_found_listener).chain(),
                ),
            );
    }
}

/// A component that makes this entity able to pathfind.
#[derive(Component, Default)]
pub struct Pathfinder {
    pub path: VecDeque<Node>,
}
#[allow(clippy::type_complexity)]
fn add_default_pathfinder(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Pathfinder>, With<Local>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(Pathfinder::default());
    }
}

pub trait PathfinderClientExt {
    fn goto(&self, goal: impl Goal + Send + Sync + 'static);
}

impl PathfinderClientExt for azalea_client::Client {
    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::{BlockPos, pathfinder::goals::BlockPosGoal};
    /// # fn example(bot: &Client) {
    /// bot.goto(BlockPosGoal::from(BlockPos::new(0, 70, 0)));
    /// # }
    /// ```
    fn goto(&self, goal: impl Goal + Send + Sync + 'static) {
        self.ecs.lock().send_event(GotoEvent {
            entity: self.entity,
            goal: Arc::new(goal),
        });
    }
}
#[derive(Event)]
pub struct GotoEvent {
    pub entity: Entity,
    pub goal: Arc<dyn Goal + Send + Sync>,
}
#[derive(Event)]
pub struct PathFoundEvent {
    pub entity: Entity,
    pub path: VecDeque<Node>,
}

#[derive(Component)]
pub struct ComputePath(Task<Option<PathFoundEvent>>);

fn goto_listener(
    mut commands: Commands,
    mut events: EventReader<GotoEvent>,
    mut query: Query<(&Position, &InstanceName)>,
    instance_container: Res<InstanceContainer>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for event in events.iter() {
        let (position, world_name) = query
            .get_mut(event.entity)
            .expect("Called goto on an entity that's not in the world");
        let start = Node {
            pos: BlockPos::from(position),
            vertical_vel: VerticalVel::None,
        };

        let world_lock = instance_container
            .get(world_name)
            .expect("Entity tried to pathfind but the entity isn't in a valid world");
        let end = event.goal.goal_node();

        let goal = event.goal.clone();
        let entity = event.entity;

        let task = thread_pool.spawn(async move {
            debug!("start: {start:?}, end: {end:?}");

            let possible_moves: Vec<&dyn moves::Move> = vec![
                &moves::ForwardMove(CardinalDirection::North),
                &moves::ForwardMove(CardinalDirection::East),
                &moves::ForwardMove(CardinalDirection::South),
                &moves::ForwardMove(CardinalDirection::West),
                //
                &moves::AscendMove(CardinalDirection::North),
                &moves::AscendMove(CardinalDirection::East),
                &moves::AscendMove(CardinalDirection::South),
                &moves::AscendMove(CardinalDirection::West),
                //
                &moves::DescendMove(CardinalDirection::North),
                &moves::DescendMove(CardinalDirection::East),
                &moves::DescendMove(CardinalDirection::South),
                &moves::DescendMove(CardinalDirection::West),
                //
                &moves::DiagonalMove(CardinalDirection::North),
                &moves::DiagonalMove(CardinalDirection::East),
                &moves::DiagonalMove(CardinalDirection::South),
                &moves::DiagonalMove(CardinalDirection::West),
            ];

            let successors = |node: &Node| {
                let mut edges = Vec::new();

                let world = world_lock.read();
                for possible_move in &possible_moves {
                    let possible_move = possible_move.get(&world, node);
                    if let Some(possible_move) = possible_move {
                        edges.push(Edge {
                            target: possible_move.node,
                            cost: possible_move.cost,
                        });
                    }
                }
                edges
            };

            // let mut pf = MTDStarLite::new(
            //     start,
            //     end,
            //     |n| goal.heuristic(n),
            //     successors,
            //     successors,
            //     |n| goal.success(n),
            // );

            let start_time = std::time::Instant::now();
            let p = a_star(
                start,
                |n| goal.heuristic(n),
                successors,
                |n| goal.success(n),
            );
            let end_time = std::time::Instant::now();
            debug!("path: {p:?}");
            debug!("time: {:?}", end_time - start_time);

            // convert the Option<Vec<Node>> to a VecDeque<Node>
            if let Some(p) = p {
                let path = p.into_iter().collect::<VecDeque<_>>();
                // commands.entity(event.entity).insert(Pathfinder { path: p });
                Some(PathFoundEvent { entity, path })
            } else {
                error!("no path found");
                None
            }
        });

        commands.spawn(ComputePath(task));
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
fn path_found_listener(mut events: EventReader<PathFoundEvent>, mut query: Query<&mut Pathfinder>) {
    for event in events.iter() {
        let mut pathfinder = query
            .get_mut(event.entity)
            .expect("Path found for an entity that doesn't have a pathfinder");
        pathfinder.path = event.path.clone();
    }
}

/// Information about our vertical velocity
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum VerticalVel {
    None,
    /// No vertical velocity, but we're not on the ground
    NoneMidair,
    // less than 3 blocks (no fall damage)
    FallingLittle,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Node {
    pub pos: BlockPos,
    pub vertical_vel: VerticalVel,
}

pub trait Goal {
    fn heuristic(&self, n: &Node) -> f32;
    fn success(&self, n: &Node) -> bool;
    // TODO: this should be removed and mtdstarlite should stop depending on
    // being given a goal node
    fn goal_node(&self) -> Node;
}

impl Node {
    /// Returns whether the entity is at the node and should start going to the
    /// next node.
    #[must_use]
    pub fn is_reached(&self, position: &Position, physics: &Physics) -> bool {
        // println!(
        //     "entity.delta.y: {} {:?}=={:?}, self.vertical_vel={:?}",
        //     entity.delta.y,
        //     BlockPos::from(entity.pos()),
        //     self.pos,
        //     self.vertical_vel
        // );
        BlockPos::from(position) == self.pos
            && match self.vertical_vel {
                VerticalVel::NoneMidair => physics.delta.y > -0.1 && physics.delta.y < 0.1,
                VerticalVel::None => physics.on_ground,
                VerticalVel::FallingLittle => physics.delta.y < -0.1,
            }
    }
}
