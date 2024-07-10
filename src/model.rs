#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use crate::item::{Item, MatchedItem};
use std::cmp;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::sync::RwLockWriteGuard;
use std::sync::{Arc, RwLock};
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct Model {
    pub query: String,
    query_cursor: i32, // > qu<query_cursor>ery
    num_matched: u64,
    num_total: u64,
    pub items: Arc<RwLock<Vec<Item>>>, // all items
    pub matched_items: Vec<MatchedItem>,
    item_cursor: usize,    // the index of matched item currently highlighted.
    line_cursor: usize,    // line No.
    item_start_pos: usize, // for screen scroll.
    max_y: i32,
    max_x: i32,
    stdout: RwLock<RawTerminal<Stdout>>,
}

impl Model {
    pub fn new(stdout: RawTerminal<Stdout>) -> Self {
        let (max_x, max_y) = termion::terminal_size().expect("failed to get terminal size");

        Model {
            query: String::new(),
            query_cursor: 0,
            num_matched: 0,
            num_total: 0,
            items: Arc::new(RwLock::new(Vec::new())),
            matched_items: Vec::new(),
            item_cursor: 0,
            line_cursor: (max_y - 3) as usize,
            item_start_pos: 0,
            max_y: max_y as i32,
            max_x: max_x as i32,
            stdout: RwLock::new(stdout),
        }
    }

    pub fn output(&self) {
        let items = self.items.read().unwrap();
        for item in items.iter() {
            if item.selected {
                println!("{}", item.text);
            }
        }
        //println!("{:?}", items[self.matched_items[self.item_cursor].index].text);
        //items[self.matched_items[self.item_cursor].index].selected = s;
    }

    pub fn toggle_select(&self, selected: Option<bool>) {
        let mut items = self.items.write().unwrap();
        items[self.matched_items[self.item_cursor].index].toggle_select(selected);
    }

    pub fn update_query(&mut self, query: String, cursor: i32) {
        self.query = query;
        self.query_cursor = cursor;
    }

    pub fn update_process_info(&mut self, matched: u64, total: u64) {
        self.num_matched = matched;
        self.num_total = total;
    }

    pub fn push_item(&mut self, item: MatchedItem) {
        self.matched_items.push(item);
    }

    pub fn clear_items(&mut self) {
        self.matched_items.clear();
    }

    pub fn move_line_cursor(&mut self, diff: i32) {
        let y = self.line_cursor as i32 + diff;
        let item_y = cmp::max(0, self.item_cursor as i32 - diff);
        let screen_height = (self.max_y - 3) as usize;

        match y {
            y if y < 0 => {
                self.line_cursor = 0;
                self.item_cursor = cmp::min(item_y as usize, self.matched_items.len() - 1);
                self.item_start_pos = self.item_cursor - screen_height;
            }

            y if y > screen_height as i32 => {
                self.line_cursor = screen_height;
                self.item_cursor = cmp::max(0, item_y as usize);
                self.item_start_pos = self.item_cursor;
            }

            y => {
                self.line_cursor = y as usize;
                self.item_cursor = item_y as usize;
            }
        }
    }

    pub fn print_query(&mut self) {
        // >  c-query
        let mut stdout = self.stdout.write().unwrap();

        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(0, self.max_y as u16 - 1),
            "> ",
            &self.query
        )
        .unwrap();
        cursor::Goto(self.query_cursor as u16 + 2, self.max_y as u16 - 1);
        stdout.flush().unwrap();
    }

    pub fn print_info(&mut self) {
        let mut stdout = self.stdout.write().unwrap();

        write!(
            stdout,
            "{}{}/{}",
            cursor::Goto(2, self.max_y as u16 - 2),
            self.num_matched,
            self.num_total
        )
        .unwrap();

        stdout.flush().unwrap();
    }

    pub fn print_items(&mut self) {
        let items = self.items.read().unwrap();

        let mut y = self.max_y as u16 - 3;
        for matched in self.matched_items[self.item_start_pos..].into_iter() {
            let mut stdout = self.stdout.write().unwrap();

            let is_current_line = y == self.line_cursor as u16;
            let item = &items[matched.index];
            let shown_str: String = item.text.chars().take((self.max_x - 1) as usize).collect();

            if is_current_line {
                write!(
                    stdout,
                    "{}{}> {}",
                    cursor::Goto(0, y),
                    color::Bg(color::LightBlack),
                    shown_str
                )
                .unwrap();
            } else {
                write!(stdout, "{}  {}", cursor::Goto(0, y), shown_str).unwrap();
            }

            write!(stdout, "{}", color::Bg(color::Reset)).unwrap();

            stdout.flush().unwrap();

            y -= 1;
            if y == 0 {
                break;
            }
        }
    }

    pub fn display(&mut self) {
        {
            let mut stdout = self.stdout.write().unwrap();

            let (_, y) = termion::terminal_size().expect("failed to get terminal size");

            for k in 0..10 {
                write!(
                    stdout,
                    "{}{}",
                    cursor::Goto(0, y - k),
                    termion::clear::CurrentLine
                )
                .unwrap();
            }
            stdout.flush().unwrap();
        }

        self.print_items();
        self.print_info();
        self.print_query();
    }
}
