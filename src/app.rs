use super::consts::*;
use super::display::handle_display;
use super::password::PasswordEntry;
use super::query::handle_query;
use super::search::Engine;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use termion::raw::IntoRawMode;

pub struct App {
    query: String,
    state: AppState,
    engine: Engine,
}

enum AppState {
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

    pub fn run(&mut self) {
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        print!("{}", CLEAR_SCREEN);
        print!("{}", HIDE_CURSOR);

        stdout.flush().unwrap();

        let (tx, rx) = mpsc::channel::<String>();

        let engine = self.engine.clone();
        let engine2 = engine.clone();
        let input_thread = thread::spawn(move || handle_query(tx, engine));
        let display_thread = thread::spawn(move || handle_display(rx, engine2));

        input_thread.join().unwrap();
        display_thread.join().unwrap();

        print!("{}", CLEAR_SCREEN);
        print!("Exiting...");
        io::stdout().flush().unwrap();
        stdout.suspend_raw_mode().unwrap();
    }
}
