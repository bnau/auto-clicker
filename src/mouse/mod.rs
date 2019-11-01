extern crate enigo;
use enigo::{Enigo, MouseButton, MouseControllable};

use crate::state::{AppState, State};

pub fn make_mouse_events(state: &State) {
    let mut enigo = Enigo::new();

    let button_type = match state.value {
        AppState::DROIT => MouseButton::Right,
        _ => MouseButton::Left,
    };

    enigo.mouse_click(button_type);
}
