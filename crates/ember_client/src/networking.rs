mod systems;

use std::net::{Ipv4Addr, SocketAddrV4};

use bevy::prelude::*;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin};

#[derive(Default)]
pub struct EmberClientNetworkPlugin;

impl Plugin for EmberClientNetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(NetworkingPlugin)
            .add_startup_system(start_system.system())
            .add_system(systems::send_message.system());
    }
}

fn start_system(mut net: ResMut<NetworkResource>) {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);

    net.bind(socket).unwrap();
}
