use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem;
/// eventbox is a simple abstract of event handling
///
/// The concept is:
///
/// An eventbox stores a vector of events, the order does not mather.
///
/// 1. the sender and receiver of a/some events share an eventbox
/// 2. when some event happans, the sender push the event/value into the eventbox.
/// 3. Meanwhile the receiver is waiting(blocked).
/// 4. When some event happen, eventbox will notify the receiver.
///
/// # Examples
/// ```
/// use std::sync::Arc;
/// use std::thread;
/// use fzf_rs::util::eventbox::EventBox;
/// let mut eb = Arc::new(EventBox::new());
/// let mut eb2 = eb.clone();
///
/// thread::spawn(move || {
///     eb2.set(10, Box::new(20));
/// });
///
/// let val: i32 = *eb.wait_for(10).downcast().unwrap();
/// assert_eq!(20, val);
/// ```
use std::sync::{Condvar, Mutex};

pub type Value = Box<dyn Any + 'static + Send>;
pub type Events<T> = HashMap<T, Value>;

struct EventData<T> {
    events: Events<T>,
}

pub struct EventBox<T> {
    mutex: Mutex<EventData<T>>,
    cond: Condvar,
}

impl<T: Hash + Eq + Copy> EventBox<T> {
    pub fn new() -> Self {
        EventBox {
            mutex: Mutex::new(EventData {
                events: HashMap::new(),
            }),
            cond: Condvar::new(),
        }
    }

    /// wait: wait for an event(any) to fire
    /// if any event is triggered, run callback on events vector
    pub fn wait(&self) -> Events<T> {
        let mut data = self.mutex.lock().unwrap();
        let events = mem::replace(&mut data.events, HashMap::new());
        let num_of_events = events.len();
        if num_of_events == 0 {
            let _unused = self.cond.wait(data);
        }
        events
    }

    /// set: fires an event
    pub fn set(&self, e: T, value: Value) {
        let mut data = self.mutex.lock().unwrap();
        {
            let val = data.events.entry(e).or_insert(Box::new(0));
            *val = value;
        }
        self.cond.notify_all();
    }

    /// clear the event map
    pub fn clear(&self) {
        let mut data = self.mutex.lock().unwrap();
        data.events.clear();
    }

    // peek at the event box to check whether event had been set or not
    pub fn peek(&self, event: T) -> bool {
        let data = self.mutex.lock().unwrap();
        data.events.contains_key(&event)
    }

    pub fn wait_for(&self, event: T) -> Value {
        'event_found: loop {
            for (e, val) in self.wait() {
                if e == event {
                    return val;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        let data = self.mutex.lock().unwrap();
        data.events.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_wait() {
        const NUM_OF_EVENTS: i32 = 4;

        // create `NUM_OF_EVENTS` threads that set the return value to
        // their thread number, the sum up the value and compare it in
        // the main thread.

        let eb = Arc::new(EventBox::new());
        let counter = Arc::new(Mutex::new(0));
        for i in 1..(NUM_OF_EVENTS + 1) {
            let eb_clone = eb.clone();
            let counter_clone = counter.clone();
            thread::spawn(move || {
                eb_clone.set(i, Box::new(i));
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
            });
        }

        // wait till all events are set
        loop {
            thread::sleep(Duration::from_millis(100));
            let count = counter.lock().unwrap();
            if *count == NUM_OF_EVENTS {
                break;
            }
        }

        let mut total: i32 = 0;
        for (_, val) in eb.wait() {
            total += *val.downcast::<i32>().unwrap();
        }

        assert_eq!((1..(NUM_OF_EVENTS + 1)).fold(0, |x, acc| acc + x), total);
    }

    #[test]
    fn test_wait_for() {
        let eb = Arc::new(EventBox::new());
        let eb2 = eb.clone();

        thread::spawn(move || {
            eb2.set(10, Box::new(20));
        });

        let val: i32 = *eb.wait_for(10).downcast().unwrap();
        assert_eq!(20, val);
    }
}
