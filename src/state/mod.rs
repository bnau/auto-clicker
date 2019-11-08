pub struct State {
    pub value: MouseAction,
}

impl State {
    pub fn new() -> Self {
        return Self {
            value: MouseAction::CLICK,
        };
    }
    pub fn update(&mut self, state: MouseAction) {
        self.value = state;
    }
}

#[derive(PartialEq)]
pub enum MouseAction {
    CLICK,
    DROIT,
    LONG,
    DOUBLE,
}
