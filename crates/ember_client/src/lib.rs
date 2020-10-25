mod networking;

use bevy::prelude::*;
use networking::{EmberClientNetworkPlugin};

pub fn run_app() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Ember".to_string(),
            width: 800,
            height: 600,
            vsync: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_default_plugins()
        .add_plugin(EmberClientNetworkPlugin)
        .run();
}
