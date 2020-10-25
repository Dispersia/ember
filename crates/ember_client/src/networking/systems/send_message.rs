use bevy::prelude::Res;
use bevy_prototype_networking_laminar::{NetworkDelivery, NetworkResource};

pub fn send_message(net: Res<NetworkResource>) {
    net.send(
        "127.0.0.1:12350".parse().unwrap(),
        b"Good",
        NetworkDelivery::ReliableSequenced(Some(1)),
    )
    .unwrap();
}
