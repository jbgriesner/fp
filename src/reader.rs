#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use crate::event::Event;
use crate::eventbox::EventBox;
use crate::item::Item;
use crate::prelude::Result;
use std::fs::DirEntry;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::{fs, io};

pub struct Reader {
    eb: Arc<EventBox<Event>>, // eventbox
    tx: Sender<Item>,         // sender to send the string read from command output
}

impl Reader {
    pub fn new(eb: Arc<EventBox<Event>>, tx: Sender<Item>) -> Self {
        Reader { eb, tx }
    }

    fn ls(&self) -> Result<impl Iterator<Item = io::Result<DirEntry>>> {
        let pwds = fs::read_dir("/home/jb/.fp/")?;
        Ok(pwds)
    }

    pub fn run(&mut self) {
        let mut pwds = self.ls().expect("command not found");

        loop {
            match pwds.next() {
                Some(entry) => {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().into_string().unwrap();
                        let _ = self.tx.send(Item::new(file_name));
                    }
                }
                None => break,
            }
            self.eb.set(Event::EvReaderNewItem, Box::new(0));
        }
        self.eb.set(Event::EvReaderFinished, Box::new(0));
    }
}
