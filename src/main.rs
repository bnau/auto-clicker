extern crate gio;
extern crate gtk;

use gtk::prelude::*;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod state;
use state::{AppState, State};

mod gui;
use gui::Gui;

mod mouse;
use mouse::make_mouse_events;

fn main() {
    gtk::init().expect("Unable to start GTK3. Error");

    let gui = Arc::new(Gui::new());

    let state = Arc::new(Mutex::new(State::new()));
    {
        let button = &gui.button_click;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(AppState::CLICK);
            gui.update_from(&state);
            Inhibit(false)
        });
    }
    {
        let button = &gui.button_right;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(AppState::DROIT);
            gui.update_from(&state);
            Inhibit(false)
        });
    }
    {
        let button = &gui.button_long;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(AppState::LONG);
            gui.update_from(&state);
            Inhibit(false)
        });
    }
    {
        let button = &gui.button_double;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(AppState::DOUBLE);
            gui.update_from(&state);
            Inhibit(false)
        });
    }
    {
        let state = Arc::clone(&state);
        thread::spawn(move || loop {
            let wait_time = Duration::from_secs(5);
            thread::sleep(wait_time);
            let state = &state.lock().unwrap();
            make_mouse_events(state);
        });
    }
    gui.start();
    gtk::main();
}
