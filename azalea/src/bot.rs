use crate::accept_resource_packs::AcceptResourcePacksPlugin;
use crate::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use crate::auto_respawn::AutoRespawnPlugin;
use crate::auto_tool::AutoToolPlugin;
use crate::container::ContainerPlugin;
use crate::ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query},
};
use azalea_client::interact::SwingArmEvent;
use azalea_client::mining::Mining;
use azalea_client::TickBroadcast;
use azalea_core::position::{BlockPos, Vec3};
use azalea_core::tick::GameTick;
use azalea_entity::direction_looking_at;
use azalea_entity::{
    clamp_look_direction, metadata::Player, EyeHeight, Jumping, LocalEntity, LookDirection,
    Position,
};
use azalea_physics::PhysicsSet;
use bevy_app::Update;
use bevy_ecs::prelude::Event;
use bevy_ecs::schedule::IntoSystemConfigs;
use futures_lite::Future;
use tracing::trace;

use crate::pathfinder::PathfinderPlugin;

#[derive(Clone, Default)]
pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LookAtEvent>()
            .add_event::<JumpEvent>()
            .add_systems(
                Update,
                (
                    insert_bot,
                    look_at_listener.before(clamp_look_direction),
                    jump_listener,
                ),
            )
            .add_systems(GameTick, stop_jumping.after(PhysicsSet));
    }
}

/// A component that clients with [`BotPlugin`] will have. If you just want to
/// check if an entity is one of our bots, you should use [`LocalEntity`].
#[derive(Default, Component)]
pub struct Bot {
    jumping_once: bool,
}

/// Insert the [`Bot`] component for any local players that don't have it.
#[allow(clippy::type_complexity)]
fn insert_bot(
    mut commands: Commands,
    mut query: Query<Entity, (Without<Bot>, With<LocalEntity>, With<Player>)>,
) {
    for entity in &mut query {
        commands.entity(entity).insert(Bot::default());
    }
}

fn stop_jumping(mut query: Query<(&mut Jumping, &mut Bot)>) {
    for (mut jumping, mut bot) in &mut query {
        if bot.jumping_once && **jumping {
            bot.jumping_once = false;
            **jumping = false;
        }
    }
}

pub trait BotClientExt {
    /// Queue a jump for the next tick.
    fn jump(&mut self);
    /// Turn the bot's head to look at the coordinate in the world.
    fn look_at(&mut self, pos: Vec3);
    /// Get a receiver that will receive a message every tick.
    fn get_tick_broadcaster(&self) -> tokio::sync::broadcast::Receiver<()>;
    /// Mine a block. This won't turn the bot's head towards the block, so if
    /// that's necessary you'll have to do that yourself with [`look_at`].
    ///
    /// [`look_at`]: crate::prelude::BotClientExt::look_at
    fn mine(&mut self, position: BlockPos) -> impl Future<Output = ()> + Send;
}

impl BotClientExt for azalea_client::Client {
    fn jump(&mut self) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(JumpEvent {
            entity: self.entity,
        });
    }

    fn look_at(&mut self, position: Vec3) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(LookAtEvent {
            entity: self.entity,
            position,
        });
    }

    /// ```
    /// # use azalea::prelude::*;
    /// # use azalea::container::WaitingForInventoryOpen;
    /// # async fn example(bot: &mut azalea::Client) {
    /// let mut ticks = bot.get_tick_broadcaster();
    /// while ticks.recv().await.is_ok() {
    ///     let ecs = bot.ecs.lock();
    ///     if ecs.get::<WaitingForInventoryOpen>(bot.entity).is_none() {
    ///         break;
    ///     }
    /// }
    /// # }
    /// ```
    fn get_tick_broadcaster(&self) -> tokio::sync::broadcast::Receiver<()> {
        let ecs = self.ecs.lock();
        let tick_broadcast = ecs.resource::<TickBroadcast>();
        tick_broadcast.subscribe()
    }

    async fn mine(&mut self, position: BlockPos) {
        self.start_mining(position);
        // vanilla sends an extra swing arm packet when we start mining
        self.ecs.lock().send_event(SwingArmEvent {
            entity: self.entity,
        });

        let mut receiver = self.get_tick_broadcaster();
        while receiver.recv().await.is_ok() {
            let ecs = self.ecs.lock();
            if ecs.get::<Mining>(self.entity).is_none() {
                break;
            }
        }
    }
}

/// Event to jump once.
#[derive(Event)]
pub struct JumpEvent {
    pub entity: Entity,
}

pub fn jump_listener(
    mut query: Query<(&mut Jumping, &mut Bot)>,
    mut events: EventReader<JumpEvent>,
) {
    for event in events.read() {
        if let Ok((mut jumping, mut bot)) = query.get_mut(event.entity) {
            **jumping = true;
            bot.jumping_once = true;
        }
    }
}

/// Make an entity look towards a certain position in the world.
#[derive(Event)]
pub struct LookAtEvent {
    pub entity: Entity,
    /// The position we want the entity to be looking at.
    pub position: Vec3,
}
pub fn look_at_listener(
    mut events: EventReader<LookAtEvent>,
    mut query: Query<(&Position, &EyeHeight, &mut LookDirection)>,
) {
    for event in events.read() {
        if let Ok((position, eye_height, mut look_direction)) = query.get_mut(event.entity) {
            let new_look_direction =
                direction_looking_at(&position.up(eye_height.into()), &event.position);
            trace!(
                "look at {:?} (currently at {:?})",
                event.position,
                **position
            );
            *look_direction = new_look_direction;
        }
    }
}

/// A [`PluginGroup`] for the plugins that add extra bot functionality to the
/// client.
pub struct DefaultBotPlugins;

impl PluginGroup for DefaultBotPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BotPlugin)
            .add(PathfinderPlugin)
            .add(ContainerPlugin)
            .add(AutoRespawnPlugin)
            .add(AcceptResourcePacksPlugin)
            .add(AutoToolPlugin)
    }
}
