extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, Grid};

use std::cell::RefCell;
use std::env::args;
use std::sync::Arc;

mod app_state;
use app_state::{AppState, State};

mod gui;
use gui::Gui;

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn build_ui(application: &gtk::Application) {
    let gui = Arc::new(Gui::new());
    let state = Arc::new(RefCell::new(State::new()));
    let glade_src = include_str!("gui.xml");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let button_click: Button = builder
        .get_object("click")
        .expect("Couldn't get buttonClick");

    window.show_all();
}

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
