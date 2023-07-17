//! A simple bot that repeats chat messages sent by other players.

use azalea::ecs::prelude::*;
use azalea::{pathfinder::goals, prelude::*};
use azalea_client::{Event, GameProfileComponent};
use azalea_entity::metadata::Player;
use azalea_entity::{Local, Position};

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let account = Account::microsoft("email").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(mut bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Tick => {
            let Some(player) = bot
                .entity_by::<(With<Player>, Without<Local>), (&GameProfileComponent,)>(
                    |profile: &&GameProfileComponent| profile.name == "py5",
                )
            else {
                return Ok(());
            };
            let pos = *bot.entity_component::<Position>(player);

            // bot.goto(goals::AndGoal(
            //     goals::RadiusGoal { pos, radius: 10. },
            //     // but not too close
            //     goals::InverseGoal(goals::RadiusGoal { pos, radius: 2. }),
            // ));
            bot.goto(goals::RadiusGoal { pos, radius: 3. });
        }
        _ => {}
    }

    Ok(())
}
