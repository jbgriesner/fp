use crate::event::KeyboardEvent::*;
use crate::event::{FuzzyPassEvent, KeyboardEvent};
use std::{
    io::{stdin, stdout, Stdin, Stdout, Write},
    path::Display,
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};
use termion::{event::Key, input::TermRead, screen};

pub fn run(sender: Sender<FuzzyPassEvent>) {
    let stdin = stdin();
    let mut query: Vec<char>;

    thread::spawn(move || {
        print!("running keyboard");
        'lop: for k in stdin.keys() {
            let event = match k.as_ref().unwrap() {
                Key::Char('\n') => ItemSelected,
                Key::Backspace => QueryChanged,
                // Key::Char('\t') => self.eb.set(Event::EvInputToggle, Box::new(true)),
                Key::Up => Up,
                Key::Down => Down,
                Key::Char(ch) => {
                    // self.add_char(*ch);
                    QueryChanged
                }
                Key::Esc => {
                    let event = Exit;
                    break 'lop;
                }
                Key::Ctrl('n') => NewPassword,
                _ => UnknownEvent,
            };
            sender.send(FuzzyPassEvent::KeyboardEvent(event)).unwrap();
        }
    });
}
