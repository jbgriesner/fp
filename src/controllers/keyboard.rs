use super::Runner;
use crate::event::FuzzyPassEvent;
use crate::event::KeyboardEvent::*;
use std::{
    convert::Infallible,
    io::stdin,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};
use termion::{event::Key, input::TermRead};

pub struct Keyboard {}

impl Runner for Keyboard {
    fn run(sender: Sender<FuzzyPassEvent>, token: &Receiver<Infallible>) {
        let stdin = stdin();
        let mut query: Vec<char> = Vec::new();

        for k in stdin.keys() {
            match token.try_recv() {
                Ok(never) => match never {},
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => break,
            }

            let event = match k.as_ref().unwrap() {
                Key::Char('\n') => {
                    query = vec![];
                    ItemSelected
                }
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
    }
}
