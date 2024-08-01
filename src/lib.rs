#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

pub use crate::prelude::{f, Error, Result};
use event::FuzzyPassEvent::{KeyboardEvent, SourceEvent};
use event::KeyboardEvent::{Down, Exit, NewPassword, QueryChanged, RestartKeyboard, Up};
use event::SourceEvent::ReadFinished;
use event::{Event, FuzzyPassEvent};
use eventbox::EventBox;
use input::Input;
use item::Item;
use matcher::Matcher;
use model::Model;
use screen::{show_new_pass, MainScreen};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{
    io::{stdin, stdout, Stdin, Stdout, Write},
    path::Display,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread,
};
use termion::screen::IntoAlternateScreen;
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
mod pass;
mod prelude;
mod screen;
mod source;
mod worker;

pub fn run() -> Result<()> {
    let (sender, receiver) = channel();
    let on_main_screen = Arc::new(AtomicBool::new(true));

    let mut app = MainScreen::new();

    source::run(sender.clone());
    keyboard::run(sender.clone());
    let mut main_screen = true;

    loop {
        // if on_main_screen.load(Ordering::SeqCst) {
        if true {
            match receiver.recv().unwrap() {
                KeyboardEvent(keyboard_event) if main_screen => match keyboard_event {
                    Exit => break,
                    QueryChanged(q) => app.set_query(q.iter().cloned().collect::<String>()),
                    Down => app.move_line_cursor_down(),
                    Up => app.move_line_cursor_up(),
                    NewPassword => {
                        let on_main_screen_clone = on_main_screen.clone();
                        let sender_clone = sender.clone();
                        thread::spawn(move || {
                            show_new_pass(on_main_screen_clone, sender_clone);
                        });
                        on_main_screen.store(true, Ordering::SeqCst);
                        continue;
                    }
                    RestartKeyboard(pwd) => {
                        println!("{}{:?}", termion::cursor::Goto(20, 20), pwd);
                        keyboard::run(sender.clone())
                    }
                    _ => break,
                },
                KeyboardEvent(_) => continue,
                SourceEvent(source_event) => match source_event {
                    ReadFinished(items) => {
                        app.set_items(items);
                    }
                },
            }
            app.show();
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
    Ok(())
}
