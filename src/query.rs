use crate::password::PasswordEntry;
use crate::search::Engine;

use super::app::App;
use super::app::AppState;
use super::consts::*;
use std::sync::Mutex;
use std::{
    io::{self, Write},
    sync::{mpsc::Sender, Arc},
};
use termion::input::TermRead;

pub fn handle_query(tx: Sender<String>, app: Arc<Mutex<App>>) {
    let mut query = String::new();

    let min_line = 4;
    let mut max_line = 4;
    let line_left = "\x1B[";
    let line_right = ";7H";
    let line_right_selected = ";1H";

    for c in io::stdin().keys() {
        let mut app = app.lock().unwrap();
        print!("{}", TOP_LEFT);

        match (c.unwrap(), app.get_state()) {
            (termion::event::Key::Ctrl('c'), _) => break,
            (termion::event::Key::Up, AppState::Selecting { current_line }) => {
                if current_line > min_line {
                    let new_current_line = current_line - 1;

                    print!("{}{}{}", line_left, current_line, line_right);
                    print!("  ");

                    *app = App {
                        query: query.clone(),
                        state: AppState::Selecting {
                            current_line: new_current_line,
                        },
                        engine: Engine {},
                    };

                    print!("{}{}{}", line_left, new_current_line, line_right);
                    print!("> ");
                    io::stdout().flush().unwrap();
                }
            }
            (termion::event::Key::Down, AppState::Selecting { current_line }) => {
                if current_line < max_line {
                    let new_current_line = current_line + 1;

                    print!("{}{}{}", line_left, current_line, line_right);
                    print!("  ");

                    *app = App {
                        query: query.clone(),
                        state: AppState::Selecting {
                            current_line: new_current_line,
                        },
                        engine: Engine {},
                    };

                    print!("{}{}{}", line_left, new_current_line, line_right);
                    print!("> ");
                    io::stdout().flush().unwrap();
                }
            }
            (termion::event::Key::Char('\n'), state) if query != "" => match state {
                AppState::Selecting { .. } => {
                    print!("{}{}{}", line_left, max_line + 1, line_right_selected);
                    print!("Selected Password: ");
                    io::stdout().flush().unwrap();

                    *app = App {
                        query: query.clone(),
                        state: AppState::Selected(PasswordEntry::new()),
                        engine: Engine {},
                    };
                }
                AppState::Searching => {
                    *app = App {
                        query: query.clone(),
                        state: AppState::Selecting {
                            current_line: min_line,
                        },
                        engine: Engine {},
                    };
                    print!("{}", FIRST_RESULT);
                    print!("> ");
                    io::stdout().flush().unwrap();
                    max_line = 3 + app.engine.search(query.clone()).len() as u32;
                }
                _ => (),
            },
            (termion::event::Key::Backspace, AppState::Searching) => {
                query.pop();
                print!("{}", CLEAR_SCREEN);
                print!("{}", HIDE_CURSOR);
                print!("{}", TOP_LEFT);
                print!("Search: {}", query);
                tx.send(query.clone()).unwrap();
            }
            (termion::event::Key::Char(ch), AppState::Searching) if ch != '\n' => {
                query.push(ch);
                print!("Search: {}", query);
                tx.send(query.clone()).unwrap();
            }
            _ => {}
        }
    }
}
