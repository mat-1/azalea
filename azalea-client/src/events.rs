//! Defines the [`Event`] enum and makes those events trigger when they're sent
//! in the ECS.

use std::sync::Arc;

use azalea_protocol::packets::game::{
    clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket, ClientboundGamePacket,
};
use azalea_world::{InstanceName, MinecraftEntityId};
use bevy_app::{App, FixedUpdate, Plugin, Update};
use bevy_ecs::{
    component::Component,
    event::EventReader,
    query::{Added, With},
    system::Query,
};
use derive_more::{Deref, DerefMut};
use tokio::sync::mpsc;

use crate::{
    chat::{ChatPacket, ChatReceivedEvent},
    packet_handling::{
        AddPlayerEvent, DeathEvent, KeepAliveEvent, PacketEvent, RemovePlayerEvent,
        UpdatePlayerEvent,
    },
    PlayerInfo,
};

// (for contributors):
// HOW TO ADD A NEW (packet based) EVENT:
// - make a struct that contains an entity field and a data field (look in
//   packet_handling.rs for examples, also you should end the struct name with
//   "Event")
// - the entity field is the local player entity that's receiving the event
// - in packet_handling, you always have a variable called player_entity that
//   you can use
// - add the event struct in the `impl Plugin for PacketHandlerPlugin`
// - to get the event writer, you have to get an
//   EventWriter<SomethingHappenedEvent> from the SystemState (the convention is
//   to end your variable with the word "events", like "something_events")
//
// - then here in this file, add it to the Event enum
// - and make an event listener system/function like the other ones and put the
//   function in the `impl Plugin for EventPlugin`

/// Something that happened in-game, such as a tick passing or chat message
/// being sent.
///
/// Note: Events are sent before they're processed, so for example game ticks
/// happen at the beginning of a tick before anything has happened.
#[derive(Debug, Clone)]
pub enum Event {
    /// Happens right after the bot switches into the Game state, but before
    /// it's actually spawned. This can be useful for setting the client
    /// information with `Client::set_client_information`, so the packet
    /// doesn't have to be sent twice.
    Init,
    /// The client is now in the world. Fired when we receive a login packet.
    Login,
    /// A chat message was sent in the game chat.
    Chat(ChatPacket),
    /// Happens 20 times per second, but only when the world is loaded.
    Tick,
    /// We received a packet from the server.
    ///
    /// ```
    /// # use azalea_client::Event;
    /// # use azalea_protocol::packets::game::ClientboundGamePacket;
    /// # async fn example(event: Event) {
    /// # match event {
    /// Event::Packet(packet) => match *packet {
    ///     ClientboundGamePacket::Login(_) => {
    ///         println!("login packet");
    ///     }
    ///     _ => {}
    /// },
    /// # _ => {}
    /// # }
    /// # }
    /// ```
    Packet(Arc<ClientboundGamePacket>),
    /// A player joined the game (or more specifically, was added to the tab
    /// list).
    AddPlayer(PlayerInfo),
    /// A player left the game (or maybe is still in the game and was just
    /// removed from the tab list).
    RemovePlayer(PlayerInfo),
    /// A player was updated in the tab list (gamemode, display
    /// name, or latency changed).
    UpdatePlayer(PlayerInfo),
    /// The client player died in-game.
    Death(Option<Arc<ClientboundPlayerCombatKillPacket>>),
    /// A `KeepAlive` packet was sent by the server.
    KeepAlive(u64),
}

/// A component that contains an event sender for events that are only
/// received by local players. The receiver for this is returned by
/// [`Client::start_client`].
///
/// [`Client::start_client`]: crate::Client::start_client
#[derive(Component, Deref, DerefMut)]
pub struct LocalPlayerEvents(pub mpsc::UnboundedSender<Event>);

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                chat_listener,
                login_listener,
                init_listener,
                packet_listener,
                add_player_listener,
                update_player_listener,
                remove_player_listener,
                death_listener,
                keepalive_listener,
            ),
        )
        .add_systems(FixedUpdate, tick_listener);
    }
}

// when LocalPlayerEvents is added, it means the client just started
fn init_listener(query: Query<&LocalPlayerEvents, Added<LocalPlayerEvents>>) {
    for local_player_events in &query {
        local_player_events.send(Event::Init).unwrap();
    }
}

// when MinecraftEntityId is added, it means the player is now in the world
fn login_listener(query: Query<&LocalPlayerEvents, Added<MinecraftEntityId>>) {
    for local_player_events in &query {
        local_player_events.send(Event::Login).unwrap();
    }
}

fn chat_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<ChatReceivedEvent>) {
    for event in events.iter() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-localplayer entities shouldn't be able to receive chat events");
        local_player_events
            .send(Event::Chat(event.packet.clone()))
            .unwrap();
    }
}

// only tick if we're in a world
fn tick_listener(query: Query<&LocalPlayerEvents, With<InstanceName>>) {
    for local_player_events in &query {
        local_player_events.send(Event::Tick).unwrap();
    }
}

fn packet_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<PacketEvent>) {
    for event in events.iter() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-localplayer entities shouldn't be able to receive add player events");
        local_player_events
            .send(Event::Packet(Arc::new(event.packet.clone())))
            .unwrap();
    }
}

fn add_player_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<AddPlayerEvent>) {
    for event in events.iter() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-localplayer entities shouldn't be able to receive add player events");
        local_player_events
            .send(Event::AddPlayer(event.info.clone()))
            .unwrap();
    }
}

fn update_player_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<UpdatePlayerEvent>,
) {
    for event in events.iter() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-localplayer entities shouldn't be able to receive update player events");
        local_player_events
            .send(Event::UpdatePlayer(event.info.clone()))
            .unwrap();
    }
}

fn remove_player_listener(
    query: Query<&LocalPlayerEvents>,
    mut events: EventReader<RemovePlayerEvent>,
) {
    for event in events.iter() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-localplayer entities shouldn't be able to receive remove player events");
        local_player_events
            .send(Event::RemovePlayer(event.info.clone()))
            .unwrap();
    }
}

pub fn death_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<DeathEvent>) {
    for event in events.iter() {
        if let Ok(local_player_events) = query.get(event.entity) {
            local_player_events
                .send(Event::Death(event.packet.clone().map(|p| p.into())))
                .unwrap();
        }
    }
}

fn keepalive_listener(query: Query<&LocalPlayerEvents>, mut events: EventReader<KeepAliveEvent>) {
    for event in events.iter() {
        let local_player_events = query
            .get(event.entity)
            .expect("Non-localplayer entities shouldn't be able to receive keepalive events");
        local_player_events
            .send(Event::KeepAlive(event.id))
            .unwrap();
    }
}
