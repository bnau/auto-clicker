use gtk::prelude::*;

use gtk::Button;

use crate::state::{MouseAction, State};

pub struct AppButton {
    pub button: Button,
    action: MouseAction,
}

impl AppButton {
    pub fn new(button: Button, action: MouseAction) -> Self {
        Self { button, action }
    }

    pub fn update(&self, state: &State) {
        match state.value {
            x if Some(self.action) == x => self.button.get_style_context().add_class("active"),
            _ => self.button.get_style_context().remove_class("active"),
        }
    }
}
