use super::app::App;
use super::consts::BELOW_INPUT;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

pub fn handle_display(rx: Receiver<String>, app: Arc<Mutex<App>>) {
    while let Ok(query) = rx.recv() {
        print!("{}", BELOW_INPUT);
        print!("Passwords:\n");

        let results = {
            let app = app.lock().unwrap();

            app.engine.search(query)
        };

        let mut line_idx = 4;
        let LINE_LEFT = "\x1B[";
        let LINE_RIGHT = ";9H";

        for result in results {
            print!("{}{}{}", LINE_LEFT, line_idx, LINE_RIGHT);
            line_idx += 1;
            println!("{}", result);
            io::stdout().flush().unwrap();
        }
    }
}
