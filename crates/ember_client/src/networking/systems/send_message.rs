use bevy::prelude::Res;
use bevy_prototype_networking_laminar::{NetworkDelivery, NetworkResource};

use crate::networking::ServerAddress;

pub fn send_message(server_address: Res<ServerAddress>, net: Res<NetworkResource>) {
    net.send(
        server_address.0,
        b"Good",
        NetworkDelivery::ReliableSequenced(Some(1)),
    )
    .unwrap();
}
