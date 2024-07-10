use crate::event::Event;
use crate::eventbox::EventBox;
use crate::item::Item;
use crate::prelude::Result;
use std::fs::{DirEntry, ReadDir};
use std::io::Read;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::{fs, io};

pub struct Reader {
    eb: Arc<EventBox<Event>>, // eventbox
    tx: Sender<Item>,         // sender to send the string read from command output
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

impl Reader {
    pub fn new(eb: Arc<EventBox<Event>>, tx: Sender<Item>) -> Self {
        Reader { eb, tx }
    }

    fn ls(&self) -> Result<Paths<ReadDir>> {
        let pwds = fs::read_dir("/home/jb/.fp/")?;
        let paths = pwds.paths();
        Ok(paths)
    }

    pub fn run(&mut self) {
        let mut pwds = self.ls().expect("command not found");

        for pwd in pwds {
            let _ = self.tx.send(Item::new(pwd));
            self.eb.set(Event::EvReaderNewItem, Box::new(0))
        }
        self.eb.set(Event::EvReaderFinished, Box::new(0));
    }
}
