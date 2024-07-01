use crate::crypto::password::PasswordEntry;
use crate::crypto::search::Engine;

pub mod display;
pub mod query;

pub struct App {
    pub query: String,
    pub state: AppState,
    pub engine: Engine,
}

#[derive(Clone)]
pub enum AppState {
    Searching,
    Selecting { current_line: u32 },
    Selected(PasswordEntry),
}

impl App {
    pub fn new() -> Self {
        App {
            query: String::new(),
            state: AppState::Searching,
            engine: Engine {},
        }
    }

    pub fn get_state(&mut self) -> AppState {
        self.state.clone()
    }
}
