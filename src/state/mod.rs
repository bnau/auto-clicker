pub struct State {
    pub value: Option<MouseAction>,
    pub button_position: ButtonPosition,
}

impl State {
    pub const fn new() -> Self {
        return Self {
            value: None,
            button_position: ButtonPosition::RELEASED,
        };
    }
    pub fn update_value(&mut self, val: Option<MouseAction>) {
        if val.is_none() || val.unwrap() != MouseAction::DOUBLE {
            self.update_pos(ButtonPosition::RELEASED)
        }
        self.value = val;
    }
    pub fn update_pos(&mut self, pos: ButtonPosition) {
        self.button_position = pos;
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum MouseAction {
    CLICK,
    DROIT,
    LONG,
    DOUBLE,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ButtonPosition {
    PRESSED,
    RELEASED,
}
