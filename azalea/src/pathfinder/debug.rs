use azalea_client::{chat::SendChatEvent, InstanceHolder};
use azalea_core::position::Vec3;
use bevy_ecs::prelude::*;

use super::ExecutingPath;

/// A component that makes bots run /particle commands while pathfinding to show
/// where they're going. This requires the bots to have server operator
/// permissions, and it'll make them spam *a lot* of commands.
///
/// ```
/// # use azalea::prelude::*;
/// # use azalea::pathfinder::PathfinderDebugParticles;
/// # #[derive(Component, Clone, Default)]
/// # pub struct State;
///
/// async fn handle(mut bot: Client, event: azalea::Event, state: State) -> anyhow::Result<()> {
///     match event {
///         azalea::Event::Init => {
///             bot.ecs
///                 .lock()
///                 .entity_mut(bot.entity)
///                 .insert(PathfinderDebugParticles);
///         }
///         _ => {}
///     }
///     Ok(())
/// }
/// ```
#[derive(Component)]
pub struct PathfinderDebugParticles;

pub fn debug_render_path_with_particles(
    mut query: Query<(Entity, &ExecutingPath, &InstanceHolder), With<PathfinderDebugParticles>>,
    // chat_events is Option because the tests don't have SendChatEvent
    // and we have to use ResMut<Events> because bevy doesn't support Option<EventWriter>
    chat_events: Option<ResMut<Events<SendChatEvent>>>,
    mut tick_count: Local<usize>,
) {
    let Some(mut chat_events) = chat_events else {
        return;
    };
    if *tick_count >= 2 {
        *tick_count = 0;
    } else {
        *tick_count += 1;
        return;
    }
    for (entity, executing_path, instance_holder) in &mut query {
        if executing_path.path.is_empty() {
            continue;
        }

        let chunks = &instance_holder.instance.read().chunks;

        let mut start = executing_path.last_reached_node;
        for (i, movement) in executing_path.path.iter().enumerate() {
            let end = movement.target;

            let start_vec3 = start.center();
            let end_vec3 = end.center();

            let step_count = (start_vec3.distance_squared_to(&end_vec3).sqrt() * 4.0) as usize;

            let target_block_state = chunks.get_block_state(&movement.target).unwrap_or_default();
            let above_target_block_state = chunks
                .get_block_state(&movement.target.up(1))
                .unwrap_or_default();
            // this isn't foolproof, there might be another block that could be mined
            // depending on the move, but it's good enough for debugging
            // purposes
            let is_mining = !super::world::is_block_state_passable(target_block_state)
                || !super::world::is_block_state_passable(above_target_block_state);

            let (r, g, b): (f64, f64, f64) = if i == 0 {
                (0., 1., 0.)
            } else if is_mining {
                (1., 0., 0.)
            } else {
                (0., 1., 1.)
            };

            // interpolate between the start and end positions
            for i in 0..step_count {
                let percent = i as f64 / step_count as f64;
                let pos = Vec3 {
                    x: start_vec3.x + (end_vec3.x - start_vec3.x) * percent,
                    y: start_vec3.y + (end_vec3.y - start_vec3.y) * percent,
                    z: start_vec3.z + (end_vec3.z - start_vec3.z) * percent,
                };
                let particle_command = format!(
                "/particle dust{{color:[{r},{g},{b}],scale:{size}}} {start_x} {start_y} {start_z} {delta_x} {delta_y} {delta_z} 0 {count}",
                size = 1,
                start_x = pos.x,
                start_y = pos.y,
                start_z = pos.z,
                delta_x = 0,
                delta_y = 0,
                delta_z = 0,
                count = 1
            );
                chat_events.send(SendChatEvent {
                    entity,
                    content: particle_command,
                });
            }

            start = movement.target;
        }
    }
}
