use bevy::prelude::{KeyCode, ResMut};
use bevy_prototype_input_map::inputmap::InputMap;

use super::ControllerMap;

pub fn setup(
    mut input_map: ResMut<InputMap>,
) {
    input_map
        .bind_keyboard_pressed(KeyCode::W, &ControllerMap::Forward.to_string())
        .bind_keyboard_pressed(KeyCode::S, &ControllerMap::Backward.to_string())
        .bind_keyboard_pressed(KeyCode::D, &ControllerMap::Right.to_string())
        .bind_keyboard_pressed(KeyCode::A, &ControllerMap::Left.to_string());
}
