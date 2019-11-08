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
            button_action(&gui, &state, MouseAction::CLICK)
        });
    }
    {
        let button = &gui.button_right.button;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            button_action(&gui, &state, MouseAction::DROIT)
        });
    }
    {
        let button = &gui.button_long.button;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button
            .connect_enter_notify_event(move |_, _| button_action(&gui, &state, MouseAction::LONG));
    }
    {
        let button = &gui.button_double.button;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button.connect_enter_notify_event(move |_, _| {
            button_action(&gui, &state, MouseAction::DOUBLE)
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

fn button_action(gui: &Arc<Gui>, state: &Arc<Mutex<State>>, action: MouseAction) -> Inhibit {
    let mut state = state.lock().unwrap();
    state.update(action);
    gui.update(&state);
    Inhibit(false)
}
