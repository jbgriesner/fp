use super::consts::*;
use super::search::Engine;
use std::{
    io::{self, stdin, Write},
    sync::mpsc::Sender,
};
use termion::input::TermRead;

pub fn handle_query(tx: Sender<String>, engine: Engine) {
    let mut query = String::new();

    let mut searching = true;
    let mut selecting = false;

    let mut current_line = 4;
    let min_line = 4;
    let mut max_line = 4;
    let line_left = "\x1B[";
    let line_right = ";7H";

    for c in io::stdin().keys() {
        print!("{}", TOP_LEFT);

        match (c.unwrap(), searching) {
            (termion::event::Key::Ctrl('c'), _) => break,
            (termion::event::Key::Up, false) => {
                if (current_line > min_line) {
                    print!("{}{}{}", line_left, current_line, line_right);
                    print!("  ");
                    current_line -= 1;
                    print!("{}{}{}", line_left, current_line, line_right);
                    print!("> ");
                    io::stdout().flush().unwrap();
                }
            }
            (termion::event::Key::Down, false) => {
                if (current_line < max_line) {
                    print!("{}{}{}", line_left, current_line, line_right);
                    print!("  ");
                    current_line += 1;
                    print!("{}{}{}", line_left, current_line, line_right);
                    print!("> ");
                    io::stdout().flush().unwrap();
                }
            }
            (termion::event::Key::Char('\n'), _) => {
                if selecting {
                    print!("{}{}{}", line_left, 12, line_right);
                    print!("Selected Password: ");
                    io::stdout().flush().unwrap();
                } else {
                    searching = false;
                    selecting = true;
                    print!("{}", FIRST_RESULT);
                    print!("> ");
                    io::stdout().flush().unwrap();
                    max_line = 3 + engine.search(query.clone()).len();
                }
            }
            (termion::event::Key::Backspace, true) => {
                query.pop();
                print!("{}", CLEAR_SCREEN);
                print!("{}", HIDE_CURSOR);
                print!("{}", TOP_LEFT);
                print!("Search: {}", query);
                tx.send(query.clone()).unwrap();
            }
            (termion::event::Key::Char(ch), true) => {
                query.push(ch);
                print!("Search: {}", query);
                tx.send(query.clone()).unwrap();
            }
            _ => {}
        }
    }
}
