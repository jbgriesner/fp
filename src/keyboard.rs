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
    let mut query: Vec<char> = Vec::new();

    thread::spawn(move || {
        for k in stdin.keys() {
            let event = match k.as_ref().unwrap() {
                Key::Char('\n') => ItemSelected,
                Key::Backspace => {
                    query.pop();
                    QueryChanged(query.clone())
                }
                Key::Up => Up,
                Key::Down => Down,
                Key::Char(ch) => {
                    query.push(*ch);
                    QueryChanged(query.clone())
                }
                Key::Esc => Exit,
                Key::Ctrl('n') => NewPassword,
                _ => UnknownEvent,
            };

            sender
                .send(FuzzyPassEvent::KeyboardEvent(event.clone()))
                .unwrap();

            if event == Exit {
                break;
            }
        }
    });
}
