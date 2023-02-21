//! A simple bot that repeats chat messages sent by other players.

use azalea_client::{
    chat::{ChatReceivedEvent, SendChatEvent},
    client::DefaultPlugins,
    protocol::resolver::resolve_address,
    Account, GameProfileComponent,
};
use azalea_ecs::{
    app::App,
    ecs::Ecs,
    event::{EventReader, EventWriter},
    system::{Commands, Query, ResMut},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_chat)
        .run();
}

fn setup(
    mut commands: Commands,
    mut resolver: ResMut<AddrResolver>,
    mut accounts: ResMut<Accounts>,
) {
    let account: AccountId = accounts.offline("bot");
    let address: AddrId = resolver.resolve("localhost");
    commands.spawn(NewClient { account, address })
}

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
