use crate::event::{FuzzyPassEvent, KeyboardEvent};
use crate::item::{Item, MatchedItem};
use crate::pass::Password;
use std::io::{stdin, stdout, Stdout, Write};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, RwLock, RwLockWriteGuard};
use std::time::Duration;
use std::{cmp, thread};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use termion::{color, cursor, style};

pub struct NewPass {
    url: String,
    password: String,
    tags: String,
}

pub struct MainScreen {
    items: Vec<Item>,         // all items
    matched_items: Vec<Item>, // matched items
    query: String,
    num_matched: u64,
    num_total: u64,
    item_cursor: usize,    // the index of matched item currently highlighted.
    line_cursor: usize,    // line No.
    item_start_pos: usize, // for screen scroll.
    max_y: i32,
    max_x: i32,
    stdout: RawTerminal<Stdout>,
    max_items_displayed: u64,
}

impl MainScreen {
    pub fn new() -> Self {
        let (max_x, max_y) = termion::terminal_size().expect("failed to get terminal size");

        MainScreen {
            query: String::new(),
            num_matched: 0,
            num_total: 0,
            items: Vec::new(),
            matched_items: Vec::new(),
            item_cursor: 0,
            line_cursor: (max_y - 3) as usize,
            item_start_pos: 0,
            max_y: max_y as i32,
            max_x: max_x as i32,
            stdout: stdout().into_raw_mode().unwrap(),
            max_items_displayed: 14,
        }
    }

    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.update_matched_items();

        self.item_cursor = 0;
        self.line_cursor = (self.max_y - 3) as usize;
    }

    pub fn set_items(&mut self, items: Vec<Item>) {
        self.items = items;
        self.num_total = self.items.len() as u64;

        // self.item_cursor = 0;
        // self.line_cursor = (self.max_y - 3) as usize;
    }

    pub fn output(&mut self) {
        for item in self.items.iter() {
            if item.selected {
                println!("{}", item.text);
            }
        }

        write!(self.stdout, "{}", cursor::Show,).unwrap();

        self.stdout.flush().unwrap();
    }

    pub fn move_line_cursor_up(&mut self) {
        if self.item_cursor == self.num_matched as usize {
            return;
        } else {
            if self.line_cursor == (self.max_y - 12) as usize {
                self.item_start_pos += 1;
            } else {
                self.line_cursor -= 1;
            }
        }
        self.item_cursor += 1;
    }

    pub fn move_line_cursor_down(&mut self) {
        if self.item_cursor == 0 {
            return;
        } else {
            if self.line_cursor == (self.max_y - 3) as usize {
                self.item_start_pos -= 1;
            } else {
                self.line_cursor += 1;
            }
            self.item_cursor -= 1;
        }
    }

    fn print_query(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(0, self.max_y as u16 - 1),
            "> ",
            &self.query
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn print_info(&mut self) {
        write!(
            self.stdout,
            "{}{}/{}",
            cursor::Goto(2, self.max_y as u16 - 2),
            self.num_matched,
            self.num_total
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    fn print_items(&mut self) {
        let mut y = self.max_y as u16 - 3;

        for item in self.matched_items.iter().skip(self.item_start_pos) {
            let is_current_line = y == self.line_cursor as u16;
            let shown_str: String = item.text.chars().take((self.max_x - 1) as usize).collect();

            if is_current_line {
                write!(
                    self.stdout,
                    "{}{}{}>{} ",
                    cursor::Goto(0, y),
                    color::Bg(color::Rgb(255, 193, 7)),
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                )
                .unwrap();
                write!(self.stdout, "{}", shown_str).unwrap();
            } else {
                write!(
                    self.stdout,
                    "{}{} {} {}",
                    cursor::Goto(0, y),
                    color::Bg(color::Rgb(255, 193, 7)),
                    color::Bg(color::Reset),
                    shown_str
                )
                .unwrap();
            }

            write!(self.stdout, "{}", color::Bg(color::Reset)).unwrap();

            self.stdout.flush().unwrap();

            y -= 1;
            let min_to_display = 1 + self.max_y as u16 - self.max_items_displayed as u16;
            if y == 0 || y <= min_to_display {
                break;
            }
        }
    }

    fn update_matched_items(&mut self) -> Vec<Item> {
        let mut temp_items = vec![];

        for item in self.items.iter() {
            if match_str(&self.query, &item.text) {
                temp_items.push(item.clone());
            }
        }
        self.num_matched = temp_items.len() as u64;
        temp_items
    }

    pub fn show(&mut self) {
        self.matched_items = self.update_matched_items();

        write!(self.stdout, "{}", cursor::Hide,).unwrap();

        for k in 0..13 {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(0, self.max_y as u16 - k),
                termion::clear::CurrentLine,
            )
            .unwrap();
        }
        self.stdout.flush().unwrap();

        self.print_items();
        self.print_info();
        self.print_query();
    }
}

fn match_str(query: &str, item: &str) -> bool {
    if query == "" {
        return true;
    }

    item.starts_with(&query)
}

pub fn show_new_pass(on_main_screen: Arc<AtomicBool>, sender: mpsc::Sender<FuzzyPassEvent>) {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut screen = stdout.into_alternate_screen().unwrap();

    write!(
        screen,
        "{}{}{}{}{}{}{}{}",
        termion::cursor::Goto(3, 4),
        color::Fg(color::Red),
        color::Bg(color::Rgb(255, 193, 7)),
        style::Bold,
        "New Password",
        color::Fg(color::Reset),
        color::Bg(color::Reset),
        style::Reset
    )
    .unwrap();

    write!(
        screen,
        "{} Password > {}",
        termion::cursor::Goto(6, 5),
        termion::cursor::BlinkingBlock
    )
    .unwrap();
    screen.flush().unwrap();

    let stdin = std::io::stdin();
    for evt in stdin.keys() {
        if let Ok(key) = evt {
            write!(screen, "{}{:?}", termion::cursor::Goto(10, 10), key).unwrap();
            screen.flush().unwrap();
            match key {
                Key::Char('q') => {
                    sender
                        .send(FuzzyPassEvent::KeyboardEvent(
                            KeyboardEvent::RestartKeyboard(Password::new()),
                        ))
                        .unwrap();
                    break;
                }
                _ => continue,
            }
        }
    }
    on_main_screen.store(true, Ordering::SeqCst);
}
