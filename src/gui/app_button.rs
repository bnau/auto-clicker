use gtk::prelude::*;

use gtk::Button;

use crate::state::{MouseAction, State};

pub struct AppButton {
    pub button: Button,
    label: String,
    action: MouseAction,
}

impl AppButton {
    pub fn new(button: Button, action: MouseAction) -> Self {
        let label = button.get_label().map_or("".to_string(), |g| g.to_string());
        Self {
            button,
            label,
            action,
        }
    }

    pub fn update(&self, state: &State) {
        match state.value {
            x if Some(self.action) == x => self.button.set_label("Actif"),
            _ => self.button.set_label(self.label.as_str()),
        }
    }
}
