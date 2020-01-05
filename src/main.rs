extern crate gio;
extern crate gtk;

use gtk::prelude::*;

use std::sync::Arc;

mod state;
use state::{MouseAction, State};

mod gui;
use gui::Gui;

mod mouse;
use mouse::make_mouse_events;

fn main() {
    gtk::init().expect("Unable to start GTK3. Error");

    let gui = Arc::new(Gui::new());

    static mut STATE: State = State::new();
    {
        let button = &gui.button_click.button;
        let gui = Arc::clone(&gui);
        button.connect_enter_notify_event(move |_, _| unsafe {
            button_action(&gui, &mut STATE, MouseAction::CLICK)
        });
    }
    {
        let button = &gui.button_right.button;
        let gui = Arc::clone(&gui);
        button.connect_enter_notify_event(move |_, _| unsafe {
            button_action(&gui, &mut STATE, MouseAction::DROIT)
        });
    }
    {
        let button = &gui.button_long.button;
        let gui = Arc::clone(&gui);
        button.connect_enter_notify_event(move |_, _| unsafe {
            button_action(&gui, &mut STATE, MouseAction::LONG)
        });
    }
    {
        let button = &gui.button_double.button;
        let gui = Arc::clone(&gui);
        button.connect_enter_notify_event(move |_, _| unsafe {
            button_action(&gui, &mut STATE, MouseAction::DOUBLE)
        });
    }
    {
        unsafe {
            make_mouse_events(&mut STATE);
        }
    }
    gui.start();
    gtk::main();
}

fn button_action(gui: &Arc<Gui>, state: &mut State, action: MouseAction) -> Inhibit {
    if state.value == Some(action) {
        state.update_value(None);
    } else {
        state.update_value(Some(action));
    }
    gui.update(&state);
    Inhibit(false)
}
