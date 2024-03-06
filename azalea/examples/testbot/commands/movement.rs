use std::time::Duration;

use azalea::{
    brigadier::prelude::*,
    entity::{EyeHeight, Position},
    pathfinder::goals::{BlockPosGoal, XZGoal},
    prelude::*,
    BlockPos, SprintDirection, WalkDirection,
};
use parking_lot::Mutex;

use crate::BotTask;

use super::{CommandSource, Ctx};

pub fn register(commands: &mut CommandDispatcher<Mutex<CommandSource>>) {
    commands.register(
        literal("goto")
            .executes(|ctx: &Ctx| {
                let mut source = ctx.source.lock();
                println!("got goto");
                // look for the sender
                let Some(entity) = source.entity() else {
                    source.reply("I can't see you!");
                    return 0;
                };
                let Some(position) = source.bot.get_entity_component::<Position>(entity) else {
                    source.reply("I can't see you!");
                    return 0;
                };
                source.reply("ok");
                source.bot.goto(BlockPosGoal(BlockPos::from(position)));
                1
            })
            .then(literal("xz").then(argument("x", integer()).then(
                argument("z", integer()).executes(|ctx: &Ctx| {
                    let source = ctx.source.lock();
                    let x = get_integer(ctx, "x").unwrap();
                    let z = get_integer(ctx, "z").unwrap();
                    println!("goto xz {x} {z}");
                    source.reply("ok");
                    source.bot.goto(XZGoal { x, z });
                    1
                }),
            )))
            .then(argument("x", integer()).then(argument("y", integer()).then(
                argument("z", integer()).executes(|ctx: &Ctx| {
                    let source = ctx.source.lock();
                    let x = get_integer(ctx, "x").unwrap();
                    let y = get_integer(ctx, "y").unwrap();
                    let z = get_integer(ctx, "z").unwrap();
                    println!("goto xyz {x} {y} {z}");
                    source.reply("ok");
                    source.bot.goto(BlockPosGoal(BlockPos::new(x, y, z)));
                    1
                }),
            ))),
    );

    commands.register(literal("down").executes(|ctx: &Ctx| {
        let source = ctx.source.clone();
        tokio::spawn(async move {
            let mut bot = source.lock().bot.clone();
            let position = BlockPos::from(bot.position());
            source.lock().reply("mining...");
            bot.mine(position.down(1)).await;
            source.lock().reply("done");
        });
        1
    }));

    commands.register(
        literal("look")
            .executes(|ctx: &Ctx| {
                // look for the sender
                let mut source = ctx.source.lock();
                let Some(entity) = source.entity() else {
                    source.reply("I can't see you!");
                    return 0;
                };
                let Some(position) = source.bot.get_entity_component::<Position>(entity) else {
                    source.reply("I can't see you!");
                    return 0;
                };
                let eye_height = source
                    .bot
                    .get_entity_component::<EyeHeight>(entity)
                    .map(|h| *h)
                    .unwrap_or_default();
                source.bot.look_at(position.up(eye_height as f64));
                1
            })
            .then(argument("x", integer()).then(argument("y", integer()).then(
                argument("z", integer()).executes(|ctx: &Ctx| {
                    let pos = BlockPos::new(
                        get_integer(ctx, "x").unwrap(),
                        get_integer(ctx, "y").unwrap(),
                        get_integer(ctx, "z").unwrap(),
                    );
                    println!("{:?}", pos);
                    let mut source = ctx.source.lock();
                    source.bot.look_at(pos.center());
                    1
                }),
            ))),
    );

    commands.register(
        literal("walk").then(argument("seconds", float()).executes(|ctx: &Ctx| {
            let mut seconds = get_float(ctx, "seconds").unwrap();
            let source = ctx.source.lock();
            let mut bot = source.bot.clone();

            if seconds < 0. {
                bot.walk(WalkDirection::Backward);
                seconds = -seconds;
            } else {
                bot.walk(WalkDirection::Forward);
            }

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
                bot.walk(WalkDirection::None);
            });
            source.reply(&format!("ok, walking for {seconds} seconds"));
            1
        })),
    );
    commands.register(
        literal("sprint").then(argument("seconds", float()).executes(|ctx: &Ctx| {
            let seconds = get_float(ctx, "seconds").unwrap();
            let source = ctx.source.lock();
            let mut bot = source.bot.clone();
            bot.sprint(SprintDirection::Forward);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
                bot.walk(WalkDirection::None);
            });
            source.reply(&format!("ok, sprinting for {seconds} seconds"));
            1
        })),
    );

    commands.register(literal("sneak").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        let mut bot = source.bot.clone();
        bot.sneak(!bot.sneaking());
        source.reply("ok");
        1
    }));

    commands.register(literal("north").executes(|ctx: &Ctx| {
        let mut source = ctx.source.lock();
        source.bot.set_direction(180., 0.);
        source.reply("ok");
        1
    }));
    commands.register(literal("south").executes(|ctx: &Ctx| {
        let mut source = ctx.source.lock();
        source.bot.set_direction(0., 0.);
        source.reply("ok");
        1
    }));
    commands.register(literal("east").executes(|ctx: &Ctx| {
        let mut source = ctx.source.lock();
        source.bot.set_direction(-90., 0.);
        source.reply("ok");
        1
    }));
    commands.register(literal("west").executes(|ctx: &Ctx| {
        let mut source = ctx.source.lock();
        source.bot.set_direction(90., 0.);
        source.reply("ok");
        1
    }));
    commands.register(
        literal("jump")
            .executes(|ctx: &Ctx| {
                let mut source = ctx.source.lock();
                source.bot.jump();
                source.reply("ok");
                1
            })
            .then(argument("enabled", bool()).executes(|ctx: &Ctx| {
                let jumping = get_bool(ctx, "enabled").unwrap();
                let mut source = ctx.source.lock();
                source.bot.set_jumping(jumping);
                1
            })),
    );

    commands.register(literal("stop").executes(|ctx: &Ctx| {
        let source = ctx.source.lock();
        source.bot.stop_pathfinding();
        source.reply("ok");
        *source.state.task.lock() = BotTask::None;
        1
    }));
}
