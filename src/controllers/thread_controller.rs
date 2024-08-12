use super::Runner;
use crate::event::FuzzyPassEvent;
use crate::event::KeyboardEvent::*;
use std::{
    convert::Infallible,
    io::stdin,
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread,
};
use termion::{event::Key, input::TermRead};

pub struct ThreadController {
    tokens: Sender<Receiver<Infallible>>,
    active_token: Option<Sender<Infallible>>,
    handle: thread::JoinHandle<()>,
}

impl ThreadController {
    pub fn spawn<T: Runner + Send + 'static>(sender: Sender<FuzzyPassEvent>) -> ThreadController {
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

                    T::run(sender_clone, &token);
                }
            }
        });

        ThreadController {
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
            let (token_sender, token_receiver) = channel::<Infallible>();
            self.active_token = Some(token_sender);
            self.tokens.send(token_receiver);
        }
    }
}
