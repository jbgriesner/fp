use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::event::Event;
use crate::eventbox::EventBox;
use std::char;
use std::io::{stdin, stdout, Write};
/// Input will listens to user input, modify the query string, send special
/// keystrokes(such as Enter, Ctrl-p, Ctrl-n, etc) to the controller.
use std::sync::Arc;

pub struct Input {
    query: Vec<char>,
    index: usize, // index in chars
    pos: usize,   // position in bytes
    eb: Arc<EventBox<Event>>,
}

impl Input {
    pub fn new(eb: Arc<EventBox<Event>>) -> Self {
        Input {
            query: Vec::new(),
            index: 0,
            pos: 0,
            eb: eb,
        }
    }

    fn get_query(&self) -> String {
        self.query.iter().cloned().collect::<String>()
    }

    fn add_char(&mut self, ch: char) {
        self.query.insert(self.index, ch);
        self.index += 1;
        self.pos += if ch.len_utf8() > 1 { 2 } else { 1 };
    }

    fn delete_char(&mut self) {
        if self.index == 0 {
            return;
        }

        let ch = self.query.remove(self.index - 1);
        self.index -= 1;
        self.pos -= if ch.len_utf8() > 1 { 2 } else { 1 };
    }

    pub fn run(&mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        'lop: for k in stdin.keys() {
            match k.as_ref().unwrap() {
                Key::Char('\n') => self.eb.set(Event::EvInputSelect, Box::new(true)),
                Key::Backspace => {
                    self.delete_char();
                    self.eb
                        .set(Event::EvQueryChange, Box::new((self.get_query(), self.pos)));
                }
                Key::Char('\t') => self.eb.set(Event::EvInputToggle, Box::new(true)),
                Key::Ctrl('p') => self.eb.set(Event::EvInputUp, Box::new(true)),
                Key::Ctrl('n') => self.eb.set(Event::EvInputDown, Box::new(true)),
                Key::Char(ch) => {
                    self.add_char(*ch);
                    self.eb
                        .set(Event::EvQueryChange, Box::new((self.get_query(), self.pos)));
                }
                Key::Esc => {
                    self.eb
                        .set(Event::Stop, Box::new((self.get_query(), self.pos)));
                    break 'lop;
                }
                _ => {
                    println!("{:?}", k)
                }
            }
            stdout.flush().unwrap();
        }
    }
}
