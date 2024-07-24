use crate::item::{Item, MatchedItem};
use std::cmp;
use std::io::{stdout, Stdout, Write};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, cursor};

pub struct Screen {
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

impl Screen {
    pub fn new() -> Self {
        let (max_x, max_y) = termion::terminal_size().expect("failed to get terminal size");

        Screen {
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
    }

    pub fn set_items(&mut self, items: Vec<Item>) {
        self.items = items;
        self.num_total = self.items.len() as u64;
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
        self.update_matched_items();

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
        self.update_matched_items();

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

    fn update_matched_items(&mut self) {
        for item in self.items.iter() {
            if match_str(&self.query, &item.text) {
                self.matched_items.push(item.clone());
            }
        }
        self.num_matched = self.matched_items.len() as u64;
    }

    pub fn show(&mut self) {
        self.update_matched_items();

        write!(self.stdout, "{}", cursor::Hide,).unwrap();

        let (_, y) = termion::terminal_size().expect("failed to get terminal size");

        for k in 0..13 {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(0, y - k),
                termion::clear::CurrentLine,
            )
            .unwrap();
        }
        self.stdout.flush().unwrap();

        self.print_items();
        self.print_info();
        self.print_query();

        self.num_matched = 0;
        self.matched_items = vec![];
    }
}

fn match_str(query: &str, item: &str) -> bool {
    if query == "" {
        return true;
    }

    item.starts_with(&query)
}
