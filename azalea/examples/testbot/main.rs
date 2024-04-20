//! A relatively simple bot for demonstrating some of Azalea's capabilities.
//!
//! Usage:
//! - Modify the consts below if necessary.
//! - Run `cargo r --example testbot`
//! - Commands are prefixed with `!` in chat. You can send them either in public
//!   chat or as a /msg.
//! - Some commands to try are `!goto`, `!killaura true`, `!down`. Check the
//!   `commands` directory to see all of them.

#![feature(async_closure)]
#![feature(trivial_bounds)]

mod commands;
pub mod killaura;

use azalea::pathfinder::PathfinderDebugParticles;
use azalea::ClientInformation;

use azalea::brigadier::command_dispatcher::CommandDispatcher;
use azalea::ecs::prelude::*;
use azalea::prelude::*;
use azalea::swarm::prelude::*;
use commands::{register_commands, CommandSource};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;

const USERNAME: &str = "azalea";
const ADDRESS: &str = "localhost";
/// The bot will only listen to commands sent by the player with this username.
const OWNER_USERNAME: &str = "py5";
/// Whether the bot should run /particle a ton of times to show where it's
/// pathfinding to. You should only have this on if the bot has operator
/// permissions, otherwise it'll just spam the server console unnecessarily.
const PATHFINDER_DEBUG_PARTICLES: bool = false;

#[tokio::main]
async fn main() {
    {
        use parking_lot::deadlock;
        use std::thread;
        use std::time::Duration;

        // Create a background thread which checks for deadlocks every 10s
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if deadlocks.is_empty() {
                continue;
            }

            println!("{} deadlocks detected", deadlocks.len());
            for (i, threads) in deadlocks.iter().enumerate() {
                println!("Deadlock #{i}");
                for t in threads {
                    println!("Thread Id {:#?}", t.thread_id());
                    println!("{:#?}", t.backtrace());
                }
            }
        });
    }

    let account = Account::offline(USERNAME);

    let mut commands = CommandDispatcher::new();
    register_commands(&mut commands);
    let commands = Arc::new(commands);

    let builder = SwarmBuilder::new();
    builder
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_account_with_state(
            account,
            State {
                commands: commands.clone(),
                ..Default::default()
            },
        )
        .join_delay(Duration::from_millis(100))
        .start(ADDRESS)
        .await
        .unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BotTask {
    #[default]
    None,
}

#[derive(Component, Clone)]
pub struct State {
    pub commands: Arc<CommandDispatcher<Mutex<CommandSource>>>,
    pub killaura: bool,
    pub task: Arc<Mutex<BotTask>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            commands: Arc::new(CommandDispatcher::new()),
            killaura: true,
            task: Arc::new(Mutex::new(BotTask::None)),
        }
    }
}

#[derive(Resource, Default, Clone)]
struct SwarmState;

async fn handle(bot: Client, event: azalea::Event, state: State) -> anyhow::Result<()> {
    match event {
        azalea::Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            })
            .await?;
            if PATHFINDER_DEBUG_PARTICLES {
                bot.ecs
                    .lock()
                    .entity_mut(bot.entity)
                    .insert(PathfinderDebugParticles);
            }
        }
        azalea::Event::Chat(chat) => {
            let (Some(username), content) = chat.split_sender_and_content() else {
                return Ok(());
            };
            if username != OWNER_USERNAME {
                return Ok(());
            }

            println!("{:?}", chat.message());

            let command = if chat.is_whisper() {
                Some(content)
            } else {
                content.strip_prefix('!').map(|s| s.to_owned())
            };
            if let Some(command) = command {
                match state.commands.execute(
                    command,
                    Mutex::new(CommandSource {
                        bot: bot.clone(),
                        chat: chat.clone(),
                        state: state.clone(),
                    }),
                ) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{err:?}");
                        let command_source = CommandSource {
                            bot,
                            chat: chat.clone(),
                            state: state.clone(),
                        };
                        command_source.reply(&format!("{err:?}"));
                    }
                }
            }
        }
        azalea::Event::Tick => {
            killaura::tick(bot.clone(), state.clone())?;

            let task = *state.task.lock();
            match task {
                BotTask::None => {}
            }
        }
        _ => {}
    }

    Ok(())
}
async fn swarm_handle(
    mut swarm: Swarm,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    match &event {
        SwarmEvent::Disconnect(account, join_opts) => {
            println!("bot got kicked! {}", account.username);
            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm
                .add_and_retry_forever_with_opts(account, State::default(), join_opts)
                .await;
        }
        SwarmEvent::Chat(chat) => {
            if chat.message().to_string() == "The particle was not visible for anybody" {
                return Ok(());
            }
            println!("{}", chat.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
