use crate::utils::consts::*;
use app::{display::handle_display, query::handle_query, App};
use prelude::Result;
use std::io::{self, Write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use termion::raw::IntoRawMode;

pub mod app;
mod crypto;
mod error;
mod utils;

pub use utils::prelude;

pub fn run(app: App) -> Result<()> {
    let app = Arc::new(Mutex::new(app));
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    print!("{}", CLEAR_SCREEN);
    print!("{}", HIDE_CURSOR);

    stdout.flush().unwrap();

    let (tx, rx) = mpsc::channel::<String>();

    let app2 = Arc::clone(&app);
    let input_thread = thread::spawn(move || handle_query(tx, Arc::clone(&app)));
    let display_thread = thread::spawn(move || handle_display(rx, app2));

    input_thread.join().unwrap();
    display_thread.join().unwrap();

    print!("{}", CLEAR_SCREEN);
    print!("Exiting...");
    io::stdout().flush().unwrap();
    stdout.suspend_raw_mode().unwrap();
    Ok(())
}
