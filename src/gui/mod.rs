use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder};

use crate::state::{MouseAction, State};

mod app_button;
use app_button::AppButton;

pub struct Gui {
    pub window: ApplicationWindow,
    pub button_click: AppButton,
    pub button_right: AppButton,
    pub button_long: AppButton,
    pub button_double: AppButton,
}

impl Gui {
    pub fn new() -> Self {
        // Initialize the UI from the Glade XML.
        let glade_src = include_str!("gui.xml");
        let builder = Builder::new_from_string(glade_src);

        // Get handles for the various controls we need to use.
        let window: ApplicationWindow = builder.get_object("window").unwrap();
        Gui {
            window,
            button_click: AppButton::new(builder.get_object("click").unwrap(), MouseAction::CLICK),
            button_right: AppButton::new(builder.get_object("droit").unwrap(), MouseAction::DROIT),
            button_long: AppButton::new(builder.get_object("long").unwrap(), MouseAction::LONG),
            button_double: AppButton::new(
                builder.get_object("double").unwrap(),
                MouseAction::DOUBLE,
            ),
        }
    }

    pub fn start(&self) {
        self.window.set_role("Auto Clicker");
        self.window.set_keep_above(true);
        self.window.set_accept_focus(false);
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        self.window.show_all();
    }

    pub fn update(&self, state: &State) {
        self.button_click.update(state);
        self.button_right.update(state);
        self.button_long.update(state);
        self.button_double.update(state);
    }
}
