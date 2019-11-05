extern crate gio;
extern crate gtk;

use gtk::prelude::*;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod state;
use state::{MouseAction, State};

mod gui;
use gui::Gui;

mod mouse;
use mouse::make_mouse_events;

fn main() {
    gtk::init().expect("Unable to start GTK3. Error");

    let gui = Arc::new(Gui::new());

    let state = Arc::new(Mutex::new(State::new()));
    {
        let button = &gui.button_click.button;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(MouseAction::CLICK);
            gui.update(&state);
            Inhibit(false)
        });
    }
    {
        let button = &gui.button_right.button;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(MouseAction::DROIT);
            gui.update(&state);
            Inhibit(false)
        });
    }
    {
        let button = &gui.button_long.button;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            let mut state = state.lock().unwrap();
            state.update(MouseAction::LONG);
            gui.update(&state);
            Inhibit(false)
        });
    }
    {
        let button = &gui.button_double.button;
        button.connect_enter_notify_event(move |_, _| {
            let gui = Arc::clone(&gui);
            let state = Arc::clone(&state);
            let mut state = state.lock().unwrap();
            state.update(MouseAction::DOUBLE);
            gui.update(&state);
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

fn update_state(state: State, gui: Gui) {}
