#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

pub use crate::prelude::{f, Error, Result};
use crate::screen::ScreenMode::*;
use event::FuzzyPassEvent::{KeyboardEvent, SourceEvent};
use event::KeyboardEvent::{
    Down, Exit, ItemSelected, NewPassword, QueryChanged, RestartKeyboard, Up,
};
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

pub mod controllers;
mod error;
mod event;
mod eventbox;
mod input;
mod item;
mod matcher;
mod model;
mod pass;
mod prelude;
mod screen;
mod source;
mod worker;

pub fn run() -> Result<()> {
    let (sender, receiver) = channel();

    let mut app = MainScreen::new();

    source::run(sender.clone());

    let mut keyboard = controllers::get_keyboard(sender.clone());
    keyboard.start();

    loop {
        match receiver.recv().unwrap() {
            KeyboardEvent(keyboard_event) if app.get_mode() == Main => match keyboard_event {
                Exit => break,
                QueryChanged(q) => app.set_query(q.iter().cloned().collect::<String>()),
                Down => app.move_line_cursor_down(),
                Up => app.move_line_cursor_up(),
                NewPassword => {
                    app.set_mode(NewPass);
                    // keyboard.pause();
                    // let sender_clone = sender.clone();
                    // thread::spawn(move || {
                    //     show_new_pass(sender_clone);
                    // });
                    // app.show_new_pass();
                    // continue;
                }
                RestartKeyboard(pwd) => {
                    println!("{}{:?}", termion::cursor::Goto(20, 20), pwd);
                    // keyboard::run(sender.clone())
                    keyboard.start();
                }
                _ => break,
            },
            KeyboardEvent(keyboard_event) if app.get_mode() == NewPass => match keyboard_event {
                Exit => app.set_mode(Main),
                QueryChanged(q) => app.update_new_pass(q.iter().cloned().collect::<String>()),
                Down => app.move_line_cursor_down(),
                Up => app.move_line_cursor_up(),
                NewPassword => {
                    keyboard.pause();
                    let sender_clone = sender.clone();
                    thread::spawn(move || {
                        show_new_pass(sender_clone);
                    });
                    continue;
                }
                ItemSelected => app.next(),
                RestartKeyboard(pwd) => {
                    println!("{}{:?}", termion::cursor::Goto(20, 20), pwd);
                    // keyboard::run(sender.clone())
                    keyboard.start();
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
    }
    Ok(())
}
