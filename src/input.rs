use std::{io::stdin, sync::Arc};

use termion::{event::Key, input::TermRead};

use crate::{event::EventBox, Event};

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
        loop {
            self.handle_char();
        }
    }

    // fetch input from curses and turn it into query.
    fn handle_char(&mut self) {
        let orig_query = self.query.clone();

        for ch in stdin().keys() {
            match ch.unwrap() {
                Key::Char(ch) => {
                    /* Enable attributes and output message. */
                    match ch {
                        '\x7F' => {
                            // backspace
                            self.delete_char();
                            self.eb.set(
                                Event::EV_QUERY_CHANGE,
                                Box::new((self.get_query(), self.pos)),
                            );
                        }
                        ch => {
                            // other characters
                            self.add_char(ch);
                            self.eb.set(
                                Event::EV_QUERY_CHANGE,
                                Box::new((self.get_query(), self.pos)),
                            );
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
