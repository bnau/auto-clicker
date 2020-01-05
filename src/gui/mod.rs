use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, CssProvider, StyleContext};

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
const CSS: &str = include_str!("gui.css");
impl Gui {
    pub fn new() -> Self {
        let glade_src = include_str!("gui.xml");
        let builder = Builder::new_from_string(glade_src);
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
        let screen = self.window.get_screen().unwrap();
        let style = CssProvider::new();
        let _ = CssProviderExt::load_from_data(&style, CSS.as_bytes());
        StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);
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
