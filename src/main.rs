extern crate gio;
extern crate gtk;

use gtk::prelude::*;

use std::cell::RefCell;
use std::sync::Arc;

mod app_state;
use app_state::{AppState, State};

mod gui;
use gui::Gui;

fn main() {
    gtk::init().expect("Unable to start GTK3. Error");

    let gui = Arc::new(Gui::new());

    let state = Arc::new(RefCell::new(State::new()));
    {
        let button1 = &gui.button1;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button1.connect_enter_notify_event(move |_, _| {
            let mut state = state.borrow_mut();
            state.update(Ok(AppState::CLICK));
            gui.update_from(&state);
            Inhibit(false)
        });
    }
    {
        let button2 = &gui.button2;
        let gui = Arc::clone(&gui);
        let state = Arc::clone(&state);
        button2.connect_enter_notify_event(move |_, _| {
            let mut state = state.borrow_mut();
            state.update(Ok(AppState::DROIT));
            gui.update_from(&state);
            Inhibit(false)
        });
    }
    gui.start();
    gtk::main();
}
