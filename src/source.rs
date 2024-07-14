use crate::event::{FuzzyPassEvent, SourceEvent};
use crate::item::Item;
use crate::Result;
use std::{
    fs::{self, ReadDir},
    io::{stdin, stdout, Stdin, Stdout, Write},
    path::Display,
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen,
};

fn ls() -> Result<Paths<ReadDir>> {
    let pwds = fs::read_dir("/home/jb/.fp/")?;
    let paths = pwds.paths();
    Ok(paths)
}

pub fn run(sender: Sender<FuzzyPassEvent>) {
    thread::spawn(move || {
        let mut items = Vec::new();

        let mut pwds = ls().expect("command not found");

        for pwd in pwds {
            items.push(Item::new(pwd));
        }
        let event = FuzzyPassEvent::SourceEvent(SourceEvent::ReadFinished(items));
        sender.send(event).unwrap();
    });
}

trait PathsIteratorExt: Sized {
    fn paths(self) -> Paths<Self>;
}

impl<ReadDirIterator> PathsIteratorExt for ReadDirIterator {
    fn paths(self) -> Paths<Self> {
        Paths::new(self)
    }
}

struct Paths<Iter> {
    iter: Iter,
}

impl<Iter> Paths<Iter> {
    fn new(iter: Iter) -> Self {
        Paths { iter }
    }
}

impl Iterator for Paths<ReadDir> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|entry| {
            if let Ok(entry) = entry {
                Some(entry.file_name().into_string().unwrap())
            } else {
                None
            }
        })
    }
}
