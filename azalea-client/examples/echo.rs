//! A simple bot that repeats chat messages sent by other players.

use azalea_client::{
    chat::{ChatReceivedEvent, SendChatEvent},
    client::DefaultPlugins,
    Account, Client, Event, GameProfileComponent,
};
use azalea_ecs::{
    app::App,
    ecs::Ecs,
    event::{EventReader, EventWriter},
    system::{Commands, Query},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(handle_chat)
        .add_startup_system(add_client)
        .add_azalea_client(&account, "localhost")
        .run();
    azalea_client::join(&account, "localhost", &mut app.world).await;
}

fn add_client(mut commands: Commands) {}

fn handle_chat(
    mut events: EventReader<ChatReceivedEvent>,
    mut send_chat_events: EventWriter<SendChatEvent>,
    query: Query<&GameProfileComponent>,
) {
    for event in events.iter() {
        let Ok(profile) = query.get(event.entity) else {
            continue;
        };
        let (Some(sender), content) = event.packet.split_sender_and_content() else {
            continue;
        };
        if sender == profile.name {
            continue; // ignore our own messages
        }
        send_chat_events.send(SendChatEvent {
            entity: event.entity,
            content: content.to_string(),
        });
    }
}
