extern crate enigo;
use crate::app_state::{AppState, State};
use enigo::{Enigo, MouseButton, MouseControllable};
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn make_mouse_events(state: Arc<RefCell<State>>) {
    let state = Arc::clone(&state);
    thread::spawn(move || loop {
        let mut state = state.borrow_mut();
        let wait_time = Duration::from_secs(5);
        let mut enigo = Enigo::new();

        thread::sleep(wait_time);

        let button_type = match state.value {
            AppState::DROIT => MouseButton::Right,
            _ => MouseButton::Left,
        };

        enigo.mouse_click(button_type);
    });
}
