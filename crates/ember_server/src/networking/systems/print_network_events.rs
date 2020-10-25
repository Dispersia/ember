use bevy::prelude::{Events, Res, ResMut};
use bevy_prototype_networking_laminar::NetworkEvent;

use crate::networking::NetworkListenerState;

pub fn print_network_events(
    mut state: ResMut<NetworkListenerState>,
    network_events: Res<Events<NetworkEvent>>,
) {
    for event in state.network_events.iter(&network_events) {
        log::info!("Received a NetworkEvent: {:?}", event);
    }
}
