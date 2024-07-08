use crate::event::EventBox;
use crate::prelude::{Error, Result};
use crate::Event;
use std::io::{stdin, BufReader};
use std::sync::mpsc::Sender;
use std::{
    io::BufRead,
    mem,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

const READER_LINES_CACHED: usize = 100;

pub struct Reader {
    cmd: Option<&'static str>, // command to invoke
    eb: Arc<EventBox<Event>>,  // eventbox
    tx: Sender<String>,        // sender to send the string read from command output
}

impl Reader {
    pub fn new(cmd: Option<&'static str>, eb: Arc<EventBox<Event>>, tx: Sender<String>) -> Self {
        Reader { cmd, eb, tx }
    }

    // invoke find comand.
    // fn get_command_output(&self) -> Result<Box<dyn BufRead>> {
    //     // find()
    //     Ok(Box::new(BufReader::new("stdout".to_string())))
    // }

    pub fn get_command_output(&self) -> Result<Box<dyn BufRead>> {
        let command = Command::new("sh")
            .arg("-c")
            .arg(self.cmd.unwrap_or("find ."))
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed command");
        let stdout = command
            .stdout
            .ok_or("command output: unwrap failed".to_owned())
            .expect("command execution failed");
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
                    self.tx.send(input).unwrap();
                }
                Err(_err) => {
                    break;
                }
            }
            self.eb.set(Event::EV_READER_NEW, Box::new(0));
        }
        self.eb.set(Event::EV_READER_FIN, Box::new(0));
    }
}

fn find() -> Result<Box<dyn BufRead>> {
    let command = Command::new("ls")
        .arg(".")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdout = command
        .stdout
        .ok_or(Error::Generic("command output: unwrap failed".to_owned()))?;
    Ok(Box::new(BufReader::new(stdout)))
}
