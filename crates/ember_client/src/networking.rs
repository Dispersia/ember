mod systems;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;
use bevy_prototype_networking_laminar::{NetworkResource, NetworkingPlugin};

#[derive(Default)]
pub struct EmberClientNetworkPlugin;

impl Plugin for EmberClientNetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        let socket = SocketAddrV4::new(loopback, 12350);

        app.add_plugin(NetworkingPlugin)
            .add_resource(ServerAddress(socket.into()))
            .add_startup_system(start_system.system())
            .add_system(systems::send_message.system());
    }
}

pub struct ServerAddress(SocketAddr);

fn start_system(mut net: ResMut<NetworkResource>) {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 0);

    net.bind(socket).unwrap();
}
