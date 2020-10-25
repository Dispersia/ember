mod systems;

use bevy::prelude::*;
use bevy_prototype_networking_laminar::{NetworkEvent, NetworkResource, NetworkingPlugin};

#[derive(Default)]
pub struct NetworkListenerState {
    network_events: EventReader<NetworkEvent>,
}

#[derive(Default)]
pub struct EmberServerNetworkPlugin;

impl Plugin for EmberServerNetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<NetworkListenerState>()
            .add_plugin(NetworkingPlugin)
            .add_startup_system(startup_system.system())
            .add_system(systems::print_network_events.system());
    }
}

fn startup_system(mut net: ResMut<NetworkResource>) {
    net.bind("127.0.0.1:12350").unwrap();
}
