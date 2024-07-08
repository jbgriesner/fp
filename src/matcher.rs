use crate::{event::EventBox, Event};
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

pub struct Matcher {
    rx_source: Receiver<String>,  // channel to retrieve strings from reader
    tx_output: Sender<String>,    // channel to send output to
    eb_req: Arc<EventBox<Event>>, // event box that recieve requests
    eb_notify: Arc<EventBox<Event>>, // event box that send out notification
    items: Vec<String>,
    item_pos: usize,
    query: String,
}

impl Matcher {
    pub fn new(
        rx_source: Receiver<String>,
        tx_output: Sender<String>,
        eb_req: Arc<EventBox<Event>>,
        eb_notify: Arc<EventBox<Event>>,
    ) -> Self {
        Matcher {
            rx_source: rx_source,
            tx_output: tx_output,
            eb_req: eb_req,
            eb_notify: eb_notify,
            items: Vec::new(),
            item_pos: 0,
            query: String::new(),
        }
    }

    pub fn process(&mut self) {
        for string in self.items[self.item_pos..].into_iter() {
            // process the matcher
            //self.tx_output.send(string.clone());
            (*self.eb_notify).set(Event::EV_MATCHER_UPDATE_PROCESS, Box::new(0));
            self.tx_output.send(string.clone());

            self.item_pos += 1;
            if (self.item_pos % 100) == 99 && !self.eb_req.is_empty() {
                break;
            }
        }
    }

    fn read_new_item(&mut self) {
        while let Ok(string) = self.rx_source.try_recv() {
            self.items.push(string);
        }
    }

    fn reset_query(&mut self, query: &str) {
        self.query.clear();
        self.query.push_str(query);
    }

    pub fn run(&mut self) {
        loop {
            for (e, val) in (*self.eb_req).wait() {
                match e {
                    Event::EV_MATCHER_NEW_ITEM => {
                        self.read_new_item();
                    }
                    Event::EV_MATCHER_RESET_QUERY => {
                        self.reset_query(&val.downcast::<String>().unwrap());
                    }
                    _ => {}
                }
            }

            self.process()
        }
    }
}
