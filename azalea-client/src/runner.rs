use std::{sync::Arc, time::Duration};

use azalea_ecs::{
    app::{App, Plugin},
    ecs::Ecs,
    schedule::Stage,
    system::Resource,
};
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use tokio::{runtime::Runtime, sync::mpsc, time::interval};

/// The plugin that makes the schedule only run when necessary, and adds the
/// [`EcsMutex`] resource to the ECS.
pub struct RunnerPlugin;
impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(azalea_ecs_runner);
    }
}

/// A resource that contains the [`Ecs`] as an `Arc<Mutex<Ecs>>`.
#[derive(Resource, Deref, DerefMut)]
pub struct EcsMutex(Arc<Mutex<Ecs>>);

/// A resource that contains a sender that can be used to run the ECS schedule
/// outside of a Minecraft tick.
#[derive(Resource, Deref, DerefMut)]
pub struct RunScheduleSender(mpsc::UnboundedSender<()>);

/// Start running the ECS loop! You must create your `App` from [`init_ecs_app`]
/// first.
pub fn azalea_ecs_runner(app: App) {
    // all resources should have been added by now so we can take the ecs from the
    // app
    let ecs = Arc::new(Mutex::new(app.world));
    let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();

    app.insert_resource(EcsMutex(ecs.clone()))
        .insert_resource(RunScheduleSender(run_schedule_sender));

    let spawned = tokio::spawn(async {
        let mut game_tick_interval = interval(Duration::from_millis(50));
        loop {
            // whenever we get an event from run_schedule_receiver or a tick happens, run
            // the schedule
            tokio::select! {
                _ = run_schedule_receiver.recv() => {}
                _ = game_tick_interval.tick() => {}
            }
            app.schedule.run(&mut ecs.lock());
        }
    });

    // make it blocking
    Runtime::new().unwrap().block_on(spawned);
}
