use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button};

use crate::app_state::{AppState, State};

pub struct Gui {
    pub window: ApplicationWindow,
    pub button1: Button,
    pub button2: Button,
}

impl Gui {
    pub fn new() -> Self {
        // Initialize the UI from the Glade XML.
        let glade_src = include_str!("gui.xml");
        let builder = Builder::new_from_string(glade_src);

        // Get handles for the various controls we need to use.
        let window: ApplicationWindow = builder.get_object("window").unwrap();
        let button1: Button = builder.get_object("click").unwrap();
        let button2: Button = builder.get_object("droit").unwrap();

        Gui {
            window,
            button1,
            button2,
        }
    }

    pub fn start(&self) {
        self.window.set_wmclass("Auto Clicker", "Auto Clicker");
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        self.window.show_all();
    }

    pub fn update_from(&self, state: &State) {
        match state.value {
            AppState::CLICK => {
                self.button1.set_label("Active");
                self.button2.set_label("Droit");
            }
            AppState::DROIT => {
                self.button1.set_label("Click");
                self.button2.set_label("Active");
            }
            _ => (),
        }
    }
}
