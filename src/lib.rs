#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

pub use crate::prelude::{f, Error, Result};
use event::{Event, FuzzyPassEvent, KeyboardEvent, SourceEvent};
use eventbox::EventBox;
use input::Input;
use item::Item;
use matcher::Matcher;
use model::Model;
use reader::Reader;
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
mod reader;
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
            FuzzyPassEvent::KeyboardEvent(keyboard_event) => match keyboard_event {
                KeyboardEvent::Exit => break,
                _ => break,
            },
            FuzzyPassEvent::SourceEvent(source_event) => match source_event {
                SourceEvent::ReadFinished(items) => {
                    screen.set_items(items);
                }
            },
        }
        screen.show();
    }

    Ok(())
}

pub fn run_old() -> Result<()> {
    let stdout = stdout().into_raw_mode().unwrap();

    let mut model = Model::new(stdout);

    let eb = Arc::new(EventBox::new());
    let (tx_source, rx_source) = channel();
    let (tx_matched, rx_matched) = channel();

    let eb_clone_reader = eb.clone();

    let eb_matcher = Arc::new(EventBox::new());
    let eb_matcher_clone = eb_matcher.clone();
    let eb_clone_matcher = eb.clone();
    let items = model.items.clone();

    let eb_clone_input = eb.clone();

    let mut reader = Reader::new(eb_clone_reader, tx_source);
    let mut matcher = Matcher::new(items, tx_matched, eb_matcher_clone, eb_clone_matcher);
    let mut input = Input::new(eb_clone_input);

    thread::spawn(move || {
        reader.run();
    });

    thread::spawn(move || {
        matcher.run();
    });

    thread::spawn(move || {
        input.run();
    });

    'outer: loop {
        for (e, val) in eb.wait() {
            match e {
                Event::EvReaderNewItem | Event::EvReaderFinished => {
                    let mut items = model.items.write().unwrap();
                    while let Ok(string) = rx_source.try_recv() {
                        items.push(string);
                    }
                    eb_matcher.set(Event::EvMatcherNewItem, Box::new(true));
                }

                Event::EvMatcherUpdateProcess => {
                    let (matched, total): (u64, u64) = *val.downcast().unwrap();
                    model.update_process_info(matched, total);

                    while let Ok(matched_item) = rx_matched.try_recv() {
                        model.push_item(matched_item);
                    }
                }

                Event::EvQueryChange => {
                    let (query, pos): (String, usize) = *val.downcast().unwrap();
                    let modified = query != model.query;
                    model.update_query(query.clone(), pos as i32);

                    if modified {
                        model.clear_items();
                        eb_matcher.set(Event::EvMatcherResetQuery, Box::new(model.query.clone()));
                    }
                }

                Event::EvInputSelect => {
                    // break out of the loop and output the selected item.
                    model.toggle_select(None);
                    break 'outer;
                }

                Event::EvInputToggle => {
                    model.toggle_select(None);
                    model.move_line_cursor(1);
                }
                Event::EvInputUp => {
                    model.move_line_cursor(-1);
                }
                Event::EvInputDown => {
                    model.move_line_cursor(1);
                }
                Event::Stop => {
                    break 'outer;
                }

                _ => {
                    print!("{}", format!("{}\n", e as i32).as_str());
                }
            }
        }
        model.display();
        // refresh();
    }

    model.output();
    Ok(())
}
