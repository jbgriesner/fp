use termion::terminal_size;

// Model: data structure for display the result
pub struct Model {
    pub query: String,
    query_cursor: i32,
    num_matched: u64,
    num_total: u64,
    matched_items: Vec<String>,
    item_start_pos: i64,
    line_cursor: i32,
    max_y: i32,
    max_x: i32,
}

impl Model {
    pub fn new() -> Self {
        let mut max_y = 0;
        let mut max_x = 0;

        let (x, y) = terminal_size().unwrap();
        max_y = y as i32;
        max_x = x as i32;

        Model {
            query: String::new(),
            query_cursor: 0,
            num_matched: 0,
            num_total: 0,
            matched_items: Vec::new(),
            item_start_pos: 0,
            line_cursor: 0,
            max_y: max_y,
            max_x: max_x,
        }
    }

    pub fn update_query(&mut self, query: String, cursor: i32) {
        self.query = query;
        self.query_cursor = cursor;
    }

    pub fn update_process_info(&mut self, matched: u64, total: u64) {
        self.num_matched = matched;
        self.num_total = total;
    }

    pub fn push_item(&mut self, item: String) {
        self.matched_items.push(item);
    }

    pub fn move_line_cursor(&mut self, diff: i32) {
        self.line_cursor += diff;
    }

    pub fn print_query(&self) {
        // > query
        print!(
            "{} > {}",
            termion::cursor::Goto(self.max_y as u16 - 1, 0),
            self.query
        );
        // mv(self.max_y - 1, 0);
        // clrtoeol();
        // printw("> ");
        // addstr(&self.query);
        // mv(self.max_y - 1, self.query_cursor + 2);
    }

    pub fn print_info(&self) {
        // mv(self.max_y - 2, 0);
        // clrtoeol();
        // printw();
        print!(
            "{}{}",
            termion::cursor::Goto(self.max_y as u16 - 1, 0),
            format!("  {}/{}", self.num_matched, self.num_total).as_str()
        );
    }

    pub fn print_items(&self) {
        let mut y = self.max_y - 2;
        for item in self.matched_items.iter() {
            // mv(y, 2);

            let shown_str: String = item.chars().take((self.max_x - 1) as usize).collect();
            // addstr(&shown_str);

            print!("{}{}", termion::cursor::Goto(y as u16, 2), &shown_str);

            y -= 1;
            if y <= 0 {
                break;
            }
        }
    }

    pub fn refresh(&self) {
        // refresh();
    }

    pub fn display(&self) {
        self.print_items();
        self.print_info();
        self.print_query();
        self.refresh();
    }
}
