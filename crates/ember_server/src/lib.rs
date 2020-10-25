mod networking;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use networking::EmberServerNetworkPlugin;
use std::time::Duration;

pub fn start_server() {
    App::build()
        .add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_millis(
            1 / 60,
        )))
        .add_plugin(EmberServerNetworkPlugin)
        .run();
}
