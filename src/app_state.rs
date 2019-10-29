pub struct State {
    pub value: AppState,
}

impl State {
    pub fn new() -> Self {
        return Self {
            value: AppState::CLICK,
        };
    }
    pub fn update(&mut self, state: AppState) {
        self.value = state;
    }
}

pub enum AppState {
    CLICK,
    DROIT,
    LONG,
    DOUBLE,
}
