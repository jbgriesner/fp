use std::{sync::Arc, thread};

trait Runnable {
    fn run(&self) -> ();
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new<Runner: Runnable + Send + Sync + 'static>(runner: Runner) -> Self {
        let thread = thread::spawn(move || runner.run());
        Self { thread }
    }
}
