//! a bot for testing new azalea features

#![feature(type_alias_impl_trait)]

use azalea::ecs::query::With;
use azalea::entity::metadata::Player;
use azalea::entity::{EyeHeight, Position};
use azalea::interact::HitResultComponent;
use azalea::inventory::ItemSlot;
use azalea::pathfinder::goals::BlockPosGoal;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::{prelude::*, swarm::prelude::*, BlockPos, GameProfileComponent, WalkDirection};
use azalea::{Account, Client, Event};
use azalea_core::Vec3;
use azalea_entity::Local;
use azalea_world::{InstanceName, MinecraftEntityId};
use bevy_ecs::query::Without;
use std::time::Duration;

#[derive(Default, Clone, Component)]
struct State {}

#[derive(Default, Clone, Resource)]
struct SwarmState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let mut accounts = Vec::new();

    for i in 0..1 {
        accounts.push(Account::offline(&format!("bot{i}")));
    }

    loop {
        let e = SwarmBuilder::new()
            .add_accounts(accounts.clone())
            .set_handler(handle)
            .set_swarm_handler(swarm_handle)
            .join_delay(Duration::from_millis(1000))
            .start("localhost")
            .await;
        // let e = azalea::ClientBuilder::new()
        //     .set_handler(handle)
        //     .start(Account::offline("bot"), "localhost")
        //     .await;
        eprintln!("{e:?}");
    }
}

async fn handle(mut bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            println!("bot init");
            // bot.set_client_information(ClientInformation {
            //     view_distance: 2,
            //     ..Default::default()
            // })
            // .await?;
        }
        Event::Login => {
            bot.chat("Hello world");
        }
        Event::Chat(m) => {
            println!("client chat message: {}", m.content());
            if m.content() == bot.profile.name {
                bot.chat("Bye");
                tokio::time::sleep(Duration::from_millis(50)).await;
                bot.disconnect();
            }
            let Some(sender) = m.username() else {
                return Ok(());
            };
            // let mut ecs = bot.ecs.lock();
            // let entity = bot
            //     .ecs
            //     .lock()
            //     .query::<&Player>()
            //     .iter(&mut ecs)
            //     .find(|e| e.name() == Some(sender));
            // let entity = bot.entity_by::<With<Player>>(|name: &Name| name == sender);
            let entity = bot.entity_by::<With<Player>, (&GameProfileComponent,)>(
                |profile: &&GameProfileComponent| {
                    println!("entity {profile:?}");
                    profile.name == sender
                },
            );
            println!("sender entity: {entity:?}");
            if let Some(entity) = entity {
                match m.content().as_str() {
                    "whereami" => {
                        let pos = bot.entity_component::<Position>(entity);
                        bot.chat(&format!("You're at {pos:?}",));
                    }
                    "whereareyou" => {
                        let pos = bot.position();
                        bot.chat(&format!("I'm at {pos:?}",));
                    }
                    "goto" => {
                        let entity_pos = bot.entity_component::<Position>(entity);
                        let target_pos: BlockPos = entity_pos.into();
                        println!("going to {target_pos:?}");
                        bot.goto(BlockPosGoal::from(target_pos));
                    }
                    "look" => {
                        let entity_pos = bot
                            .entity_component::<Position>(entity)
                            .up(bot.entity_component::<EyeHeight>(entity).into());
                        println!("entity_pos: {entity_pos:?}");
                        bot.look_at(entity_pos);
                    }
                    "jump" => {
                        bot.set_jumping(true);
                    }
                    "walk" => {
                        bot.walk(WalkDirection::Forward);
                    }
                    "stop" => {
                        bot.set_jumping(false);
                        bot.walk(WalkDirection::None);
                    }
                    "lag" => {
                        std::thread::sleep(Duration::from_millis(1000));
                    }
                    "inventory" => {
                        println!("inventory: {:?}", bot.menu());
                    }
                    "findblock" => {
                        let target_pos = bot
                            .world()
                            .read()
                            .find_block(bot.position(), &azalea::Block::DiamondBlock.into());
                        bot.chat(&format!("target_pos: {target_pos:?}",));
                    }
                    "gotoblock" => {
                        let target_pos = bot
                            .world()
                            .read()
                            .find_block(bot.position(), &azalea::Block::DiamondBlock.into());
                        if let Some(target_pos) = target_pos {
                            // +1 to stand on top of the block
                            bot.goto(BlockPosGoal::from(target_pos.up(1)));
                        } else {
                            bot.chat("no diamond block found");
                        }
                    }
                    "mineblock" => {
                        let target_pos = bot
                            .world()
                            .read()
                            .find_block(bot.position(), &azalea::Block::DiamondBlock.into());
                        if let Some(target_pos) = target_pos {
                            // +1 to stand on top of the block
                            bot.chat("ok mining diamond block");
                            bot.look_at(target_pos.center());
                            bot.mine(target_pos).await;
                            bot.chat("finished mining");
                        } else {
                            bot.chat("no diamond block found");
                        }
                    }
                    "lever" => {
                        let target_pos = bot
                            .world()
                            .read()
                            .find_block(bot.position(), &azalea::Block::Lever.into());
                        let Some(target_pos) = target_pos else {
                            bot.chat("no lever found");
                            return Ok(());
                        };
                        bot.goto(BlockPosGoal::from(target_pos));
                        bot.look_at(target_pos.center());
                        bot.block_interact(target_pos);
                    }
                    "hitresult" => {
                        let hit_result = bot.get_component::<HitResultComponent>();
                        bot.chat(&format!("hit_result: {hit_result:?}",));
                    }
                    "chest" => {
                        let target_pos = bot
                            .world()
                            .read()
                            .find_block(bot.position(), &azalea::Block::Chest.into());
                        let Some(target_pos) = target_pos else {
                            bot.chat("no chest found");
                            return Ok(());
                        };
                        bot.look_at(target_pos.center());
                        let container = bot.open_container(target_pos).await;
                        println!("container: {:?}", container);
                        if let Some(container) = container {
                            if let Some(contents) = container.contents() {
                                for item in contents {
                                    if let ItemSlot::Present(item) = item {
                                        println!("item: {:?}", item);
                                    }
                                }
                            } else {
                                println!("container was immediately closed");
                            }
                        } else {
                            println!("no container found");
                        }
                    }
                    "attack" => {
                        let mut nearest_entity = None;
                        let mut nearest_distance = f64::INFINITY;
                        let mut nearest_pos = Vec3::default();
                        let bot_position = bot.position();
                        let bot_instance_name = bot.component::<InstanceName>();
                        {
                            let mut ecs = bot.ecs.lock();
                            let mut query = ecs.query_filtered::<(
                                &MinecraftEntityId,
                                &Position,
                                &InstanceName,
                                &EyeHeight,
                            ), (With<MinecraftEntityId>, Without<Local>)>(
                            );
                            for (&entity_id, position, instance_name, eye_height) in
                                query.iter(&ecs)
                            {
                                if instance_name != &bot_instance_name {
                                    continue;
                                }

                                let distance = bot_position.distance_to(position);
                                if distance < 4.0 && distance < nearest_distance {
                                    nearest_entity = Some(entity_id);
                                    nearest_distance = distance;
                                    nearest_pos = position.up(**eye_height as f64);
                                }
                            }
                        }
                        if let Some(nearest_entity) = nearest_entity {
                            bot.look_at(nearest_pos);
                            bot.attack(nearest_entity);
                            bot.chat("attacking");
                            let mut ticks = bot.get_tick_broadcaster();
                            while ticks.recv().await.is_ok() {
                                if !bot.has_attack_cooldown() {
                                    break;
                                }
                            }
                            bot.chat("finished attacking");
                        } else {
                            bot.chat("no entities found");
                        }
                    }
                    _ => {}
                }
            }
        }
        Event::Packet(packet) => match *packet {
            ClientboundGamePacket::Login(_) => {
                println!("login packet");
            }
            _ => {}
        },
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
        SwarmEvent::Disconnect(account) => {
            println!("bot got kicked! {}", account.username);
            tokio::time::sleep(Duration::from_secs(5)).await;
            swarm
                .add_with_exponential_backoff(account, State::default())
                .await;
        }
        SwarmEvent::Chat(m) => {
            println!("swarm chat message: {}", m.message().to_ansi());
            if m.message().to_string() == "<py5> world" {
                for (name, world) in &swarm.instance_container.read().worlds {
                    println!("world name: {name}");
                    if let Some(w) = world.upgrade() {
                        for chunk_pos in w.read().chunks.chunks.values() {
                            println!("chunk: {chunk_pos:?}");
                        }
                    } else {
                        println!("nvm world is gone");
                    }
                }
            }
            if m.message().to_string() == "<py5> hi" {
                for bot in swarm {
                    bot.chat("hello");
                }
            }
        }
        _ => {}
    }
    Ok(())
}
