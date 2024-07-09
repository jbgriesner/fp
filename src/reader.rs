use crate::event::Event;
use crate::eventbox::EventBox;
use crate::item::Item;
use std::error::Error;
use std::io::{stdin, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::sync::Arc;

pub struct Reader {
    eb: Arc<EventBox<Event>>, // eventbox
    tx: Sender<Item>,         // sender to send the string read from command output
}

impl Reader {
    pub fn new(eb: Arc<EventBox<Event>>, tx: Sender<Item>) -> Self {
        Reader { eb, tx }
    }

    // invoke find comand.
    fn get_command_output(&self) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        let command = Command::new("sh")
            .arg("-c")
            .arg("find ./  -printf \"%f\n\".")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let stdout = command
            .stdout
            .ok_or("command output: unwrap failed".to_owned())
            .unwrap();
        Ok(Box::new(BufReader::new(stdout)))
    }

    pub fn run(&mut self) {
        let mut read = self.get_command_output().expect("command not found");

        loop {
            let mut input = String::new();
            match read.read_line(&mut input) {
                Ok(n) => {
                    if n <= 0 {
                        break;
                    }

                    if input.ends_with("\n") {
                        input.pop();
                        if input.ends_with("\r") {
                            input.pop();
                        }
                    }
                    let _ = self.tx.send(Item::new(input));
                }
                Err(_err) => {
                    break;
                }
            }
            self.eb.set(Event::EvReaderNewItem, Box::new(0));
        }
        self.eb.set(Event::EvReaderFinished, Box::new(0));
    }
}
