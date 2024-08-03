use crate::event::KeyboardEvent::*;
use std::{
    convert::Infallible,
    io::stdin,
    sync::mpsc::{self, channel, Receiver, Sender, TryRecvError},
    thread,
};
use termion::{event::Key, input::TermRead};

use crate::event::FuzzyPassEvent;

trait Runner {
    fn run();
    fn pause(&self);
    fn unpause(&self);
}

pub struct Keyboard {
    tokens: Sender<Receiver<Infallible>>,
    active_token: Option<Sender<Infallible>>,
    handle: thread::JoinHandle<()>,
}

impl Keyboard {
    pub fn spawn(sender: Sender<FuzzyPassEvent>) -> Keyboard {
        let (tokens_sender, tokens_receiver) = channel::<Receiver<Infallible>>();
        let sender_clone = sender.clone();

        let handle = std::thread::spawn(move || {
            for token in tokens_receiver {
                loop {
                    match token.try_recv() {
                        Ok(never) => match never {},
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => break,
                    }
                    let sender_clone = sender_clone.clone();

                    // Pass the token onto the work for finer-grained cancellation.
                    // do_work(&token);
                    run(sender_clone, &token);
                }
            }
        });

        Keyboard {
            tokens: tokens_sender,
            active_token: None,
            handle,
        }
    }

    pub fn pause(&mut self) {
        if self.active_token.is_some() {
            self.active_token = None;
        }
    }

    pub fn start(&mut self) {
        if self.active_token.is_none() {
            let (token_sender, token_receiver) = mpsc::channel::<Infallible>();
            self.active_token = Some(token_sender);
            self.tokens.send(token_receiver);
        }
    }
}

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
}
