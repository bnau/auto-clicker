use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button};

use crate::app_state::{AppState, State};

pub struct Gui {
    pub window: ApplicationWindow,
    pub button_click: Button,
    pub button_right: Button,
    pub button_long: Button,
    pub button_double: Button,
}

impl Gui {
    pub fn new() -> Self {
        // Initialize the UI from the Glade XML.
        let glade_src = include_str!("gui.xml");
        let builder = Builder::new_from_string(glade_src);

        // Get handles for the various controls we need to use.
        let window: ApplicationWindow = builder.get_object("window").unwrap();
        let button_click: Button = builder.get_object("click").unwrap();
        let button_right: Button = builder.get_object("droit").unwrap();
        let button_long: Button = builder.get_object("long").unwrap();
        let button_double: Button = builder.get_object("double").unwrap();

        Gui {
            window,
            button_click,
            button_right,
            button_long,
            button_double,
        }
    }

    pub fn start(&self) {
        self.window.set_role("Auto Clicker");
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        self.window.show_all();
    }

    pub fn update_from(&self, state: &State) {
        self.set_default_button_labels();
        match state.value {
            AppState::CLICK => {
                self.button_click.set_label("Active");
            }
            AppState::DROIT => {
                self.button_right.set_label("Active");
            }
            AppState::LONG => {
                self.button_long.set_label("Active");
            }
            AppState::DOUBLE => {
                self.button_double.set_label("Active");
            }
        }
    }

    fn set_default_button_labels(&self) {
        self.button_click.set_label("Click");
        self.button_right.set_label("Droit");
        self.button_long.set_label("Long");
        self.button_double.set_label("Double");
    }
}
