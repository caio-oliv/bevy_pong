use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};

use crate::game::resource::UserGamepad;

pub fn setup_gamepad_connection(
    mut gamepad_event: EventReader<GamepadConnectionEvent>,
    mut user_gamepad: ResMut<UserGamepad>,
) {
    for event in gamepad_event.read() {
        if event.connected() {
            user_gamepad.add_gamepad(event.gamepad);
        } else {
            user_gamepad.remove_gamepad(event.gamepad);
        }
    }
}
