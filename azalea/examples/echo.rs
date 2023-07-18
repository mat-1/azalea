//! A simple bot that repeats chat messages sent by other players.

use azalea::prelude::*;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .add_plugins(DebugdumpPlugin)
        .start(account, "localhost")
        .await
        .unwrap();
}

pub struct DebugdumpPlugin;
impl azalea::app::Plugin for DebugdumpPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        bevy_mod_debugdump::print_schedule_graph(app, bevy_app::FixedUpdate);
        std::process::exit(0);
    }
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            if let (Some(sender), content) = m.split_sender_and_content() {
                if sender == bot.profile.name {
                    return Ok(()); // ignore our own messages
                }
                bot.chat(&content);
            };
        }
        _ => {}
    }

    Ok(())
}
