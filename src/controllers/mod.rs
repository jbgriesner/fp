use crate::event::FuzzyPassEvent;
use keyboard::Keyboard;
use std::{
    convert::Infallible,
    sync::mpsc::{Receiver, Sender},
    thread,
};
use thread_controller::ThreadController;

mod keyboard;
mod thread_controller;

pub trait Runner {
    fn run(sender: Sender<FuzzyPassEvent>, token: &Receiver<Infallible>);
}

pub fn get_keyboard(sender: Sender<FuzzyPassEvent>) -> ThreadController {
    ThreadController::spawn::<Keyboard>(sender)
}
