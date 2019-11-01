extern crate enigo;
use enigo::{Enigo, MouseButton, MouseControllable};

use crate::state::{AppState, State};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn make_mouse_events(state: Arc<Mutex<State>>) {
    let state = Arc::clone(&state);
    thread::spawn(move || loop {
        let wait_time = Duration::from_secs(5);
        let mut enigo = Enigo::new();

        thread::sleep(wait_time);

        let button_type = match state.lock().unwrap().value {
            AppState::DROIT => MouseButton::Right,
            _ => MouseButton::Left,
        };

        enigo.mouse_click(button_type);
    });
}
