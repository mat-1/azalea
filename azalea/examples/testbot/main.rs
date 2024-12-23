//! A relatively simple bot for demonstrating some of Azalea's capabilities.
//!
//! Usage:
//! - Modify the consts below if necessary.
//! - Run `cargo r --example testbot -- --owner <owner> --name <username/email>
//!   --address <address>`.
//! - Commands are prefixed with `!` in chat. You can send them either in public
//!   chat or as a /msg.
//! - Some commands to try are `!goto`, `!killaura true`, `!down`. Check the
//!   `commands` directory to see all of them.

#![feature(async_closure)]
#![feature(trivial_bounds)]

mod commands;
pub mod killaura;

use std::time::Duration;
use std::{env, process};
use std::{sync::Arc, thread};

use azalea::brigadier::command_dispatcher::CommandDispatcher;
use azalea::ecs::prelude::*;
use azalea::pathfinder::PathfinderDebugParticles;
use azalea::prelude::*;
use azalea::swarm::prelude::*;
use azalea::ClientInformation;
use commands::{register_commands, CommandSource};
use parking_lot::Mutex;

/// Whether the bot should run /particle a ton of times to show where it's
/// pathfinding to. You should only have this on if the bot has operator
/// permissions, otherwise it'll just spam the server console unnecessarily.
const PATHFINDER_DEBUG_PARTICLES: bool = false;

#[tokio::main]
async fn main() {
    let args = parse_args();

    thread::spawn(deadlock_detection_thread);

    let account = if args.name.contains('@') {
        Account::microsoft(&args.name).await.unwrap()
    } else {
        Account::offline(&args.name)
    };

    let mut commands = CommandDispatcher::new();
    register_commands(&mut commands);

    let join_address = args.address.clone();

    let builder = SwarmBuilder::new();
    builder
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_account_with_state(account, State::new(args, commands))
        .join_delay(Duration::from_millis(100))
        .start(join_address)
        .await
        .unwrap();
}

/// Runs a loop that checks for deadlocks every 10 seconds.
///
/// Note that this requires the `deadlock_detection` parking_lot feature to be
/// enabled, which is only enabled in azalea by default when running in debug
/// mode.
fn deadlock_detection_thread() {
    loop {
        thread::sleep(Duration::from_secs(10));
        let deadlocks = parking_lot::deadlock::check_deadlock();
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
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BotTask {
    #[default]
    None,
}

#[derive(Component, Clone, Default)]
pub struct State {
    pub args: Args,
    pub commands: Arc<CommandDispatcher<Mutex<CommandSource>>>,
    pub killaura: bool,
    pub task: Arc<Mutex<BotTask>>,
}

impl State {
    fn new(args: Args, commands: CommandDispatcher<Mutex<CommandSource>>) -> Self {
        Self {
            args,
            commands: Arc::new(commands),
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
            if username != state.args.owner {
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

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub owner: String,
    pub name: String,
    pub address: String,
}

fn parse_args() -> Args {
    let mut owner_username = None;
    let mut bot_username = None;
    let mut address = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--owner" | "-O" => {
                owner_username = args.next();
            }
            "--name" | "-N" => {
                bot_username = args.next();
            }
            "--address" | "-A" => {
                address = args.next();
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                process::exit(1);
            }
        }
    }

    Args {
        owner: owner_username.unwrap_or_else(|| "admin".to_string()),
        name: bot_username.unwrap_or_else(|| "azalea".to_string()),
        address: address.unwrap_or_else(|| "localhost".to_string()),
    }
}
