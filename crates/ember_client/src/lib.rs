mod input;
mod networking;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_prototype_input_map::{inputmap::InputMap, InputMapPlugin};
use input::ControllerMap;
use networking::EmberClientNetworkPlugin;

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
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(input::setup.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
        .run();
}
