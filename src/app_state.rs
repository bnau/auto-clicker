pub struct State {
    pub value: AppState,
}

impl State {
    pub fn new() -> Self {
        return Self {
            value: AppState::CLICK,
        };
    }
    pub fn update(&mut self, r: Result<AppState, Box<std::error::Error>>) {
        match r {
            //Err(e) => self.error = Some(format!("{}", e)),
            Ok(v) => {
                self.value = v;
            }
            _ => (),
        }
    }
}

pub enum AppState {
    CLICK,
    DROIT,
    LONG,
    DOUBLE,
}
