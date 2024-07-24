#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

pub use crate::prelude::{f, Error, Result};
use event::FuzzyPassEvent::{KeyboardEvent, SourceEvent};
use event::KeyboardEvent::{Down, Exit, QueryChanged, Up};
use event::SourceEvent::ReadFinished;
use event::{Event, FuzzyPassEvent};
use eventbox::EventBox;
use input::Input;
use item::Item;
use matcher::Matcher;
use model::Model;
use screen::Screen;
use std::{
    io::{stdin, stdout, Stdin, Stdout, Write},
    path::Display,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

mod error;
mod event;
mod eventbox;
mod input;
mod item;
mod keyboard;
mod matcher;
mod model;
mod prelude;
mod screen;
mod source;
mod worker;

pub fn run() -> Result<()> {
    let (sender, receiver) = channel();

    let mut screen = Screen::new();

    source::run(sender.clone());
    keyboard::run(sender.clone());

    loop {
        match receiver.recv().unwrap() {
            KeyboardEvent(keyboard_event) => match keyboard_event {
                Exit => break,
                QueryChanged(q) => screen.set_query(q.iter().cloned().collect::<String>()),
                Down => screen.move_line_cursor_down(),
                Up => screen.move_line_cursor_up(),
                _ => break,
            },
            SourceEvent(source_event) => match source_event {
                ReadFinished(items) => {
                    screen.set_items(items);
                }
            },
        }
        screen.show();
    }
    Ok(())
}
