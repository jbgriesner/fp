#![allow(dead_code)]
#![allow(unused_variables)]

use event::EventBox;
use input::Input;
use matcher::Matcher;
use model::Model;
use prelude::Result;
use reader::Reader;
use std::io;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

mod app;
mod error;
mod event;
mod input;
mod matcher;
mod model;
mod prelude;
mod reader;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Event {
    EV_READER_NEW,
    EV_READER_FIN,
    EV_MATCHER_NEW_ITEM,
    EV_MATCHER_RESET_QUERY,
    EV_MATCHER_UPDATE_PROCESS,
    EV_MATCHER_FINISHED,
    EV_QUERY_MOVE_CURSOR,
    EV_QUERY_CHANGE,
    EV_INPUT_TOGGLE,
    EV_INPUT_UP,
    EV_INPUT_DOWN,
    EV_INPUT_SELECT,
}

struct App<R, W: Write> {
    /// The x coordinate.
    x: u16,
    /// The y coordinate.
    y: u16,
    /// Standard output.
    stdout: W,
    /// Standard input.
    stdin: R,
}

impl<R: Read, W: Write> App<R, W> {
    fn new(stdin: R, stdout: W) -> App<R, RawTerminal<W>> {
        let (x, y) = termion::terminal_size().expect("Could not get terminal size");
        App {
            x,
            y,
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
        }
    }

    /// Start the game loop.
    ///
    /// This will listen to events and do the appropriate actions.
    pub fn start(&mut self) -> Result<()> {
        self.init();

        // loop {
        //     // Read a single byte from stdin.
        //     let mut b = [0];
        //     self.stdin.read(&mut b).unwrap();

        //     match b[0] {
        //         b'h' | b'a' => self.slide(Direction::Left),
        //         b'j' | b's' => self.slide(Direction::Down),
        //         b'k' | b'w' => self.slide(Direction::Up),
        //         b'l' | b'd' => self.slide(Direction::Right),
        //         b'q' => return,
        //         _ => {}
        //     }

        //     self.stdout.flush().unwrap();
        // }
        Ok(())
    }

    fn init(&mut self) {
        for k in 0..11 {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(1, self.y - k),
                clear::CurrentLine
            )
            .unwrap();
        }

        // let mut width_counted = false;

        // for &i in self.map {
        //     if i == b'\n' {
        //         width_counted = true;
        //         self.stdout.write(b"\n\r").unwrap();
        //         if !width_counted {
        //             width_counted = true;
        //         }
        //     } else {
        //         self.stdout.write(&[i]).unwrap();
        //     }
        //     if !width_counted {
        //         self.width += 1;
        //     }
        // }
        self.update();
    }

    /// Move the cursor to the player position.
    fn update(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(self.x + 1, self.y + 1)).unwrap();
        self.stdout.flush().unwrap();
    }
}

pub fn run() -> Result<()> {
    let stdout = io::stdout();
    let mut fp = App::new(io::stdin(), stdout);
    fp.start()

    // let mut stdout = stdout.lock().into_raw_mode().unwrap();

    // let mut stdout = stdout().into_raw_mode().unwrap();

    // stdout.flush().unwrap();

    // let mut model = Model::new();

    // let eb = Arc::new(EventBox::new());
    // let (tx_source, rx_source) = channel();
    // let (tx_matched, rx_matched) = channel();

    // let eb_clone_reader = eb.clone();
    // let mut reader = Reader::new(Some(&"find ."), eb_clone_reader, tx_source);

    // let eb_matcher = Arc::new(EventBox::new());
    // let eb_matcher_clone = eb_matcher.clone();
    // let eb_clone_matcher = eb.clone();
    // let mut matcher = Matcher::new(rx_source, tx_matched, eb_matcher_clone, eb_clone_matcher);

    // let eb_clone_input = eb.clone();
    // let mut input = Input::new(eb_clone_input);

    // // start running
    // thread::spawn(move || {
    //     reader.run();
    // });

    // thread::spawn(move || {
    //     matcher.run();
    // });

    // thread::spawn(move || {
    //     input.run();
    // });

    // let mut count = 0;
    // 'outer: loop {
    //     for (e, val) in eb.wait() {
    //         count += 1;
    //         if count >= 100 {
    //             break 'outer;
    //         }
    //         match e {
    //             Event::EV_READER_NEW => {
    //                 //printw("READER_NEW!\n");
    //                 eb_matcher.set(Event::EV_MATCHER_NEW_ITEM, Box::new(true));
    //             }

    //             Event::EV_READER_FIN => {
    //                 print!("READER_FIN\n");
    //             }

    //             Event::EV_MATCHER_UPDATE_PROCESS => {
    //                 while let Ok(string) = rx_matched.try_recv() {
    //                     model.push_item(string);
    //                 }
    //             }

    //             Event::EV_QUERY_CHANGE => {
    //                 let (query, pos): (String, usize) = *val.downcast().unwrap();
    //                 let modified = query == model.query;
    //                 model.update_query(query.clone(), pos as i32);

    //                 if modified {
    //                     eb_matcher
    //                         .set(Event::EV_MATCHER_RESET_QUERY, Box::new(model.query.clone()));
    //                 }
    //             }

    //             _ => {
    //                 print!("{}", format!("{}\n", e as i32).as_str());
    //             }
    //         }
    //     }
    //     model.display();
    //     // refresh();
    // }
}
