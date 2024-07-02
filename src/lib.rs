#[macro_use]
extern crate lazy_static;

use crate::options::FuzzyPassOptions;
use crate::output::FuzzyOutput;
use crate::utils::consts::*;
use app::{display::handle_display, query::handle_query, App};
use crossbeam::channel::{Receiver, Sender};
use event::{EventReceiver, EventSender};
use prelude::Result;
use reader::Reader;
use std::any::Any;
use std::borrow::Cow;
use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use termion::raw::IntoRawMode;
use tuikit::prelude::{Event as TermEvent, *};
pub use utils::prelude;

pub mod app;
mod crypto;
mod error;
mod event;
mod input;
mod model;
mod options;
mod output;
mod reader;
mod utils;

pub type FuzzyPassSender = Sender<Arc<dyn FuzzyPassItem>>;
pub type FuzzyPassReceiver = Receiver<Arc<dyn FuzzyPassItem>>;

pub struct FuzzyPass {}

impl FuzzyPass {
    pub fn run_with(
        options: &FuzzyPassOptions,
        source: Option<FuzzyPassReceiver>,
    ) -> Option<FuzzyOutput> {
        let min_height = options
            .min_height
            .map(FuzzyPass::parse_height_string)
            .expect("min_height should have default values");
        let height = options
            .height
            .map(FuzzyPass::parse_height_string)
            .expect("height should have default values");

        let (tx, rx): (EventSender, EventReceiver) = channel();
        let term = Arc::new(
            Term::with_options(
                TermOptions::default()
                    .min_height(min_height)
                    .height(height)
                    .clear_on_exit(!options.no_clear)
                    .disable_alternate_screen(options.no_clear_start)
                    .clear_on_start(!options.no_clear_start)
                    .hold(options.select1 || options.exit0 || options.sync),
            )
            .unwrap(),
        );
        if !options.no_mouse {
            let _ = term.enable_mouse_support();
        }

        //------------------------------------------------------------------------------
        // input
        let mut input = input::Input::new();
        input.parse_keymaps(&options.bind);
        input.parse_expect_keys(options.expect.as_deref());

        let tx_clone = tx.clone();
        let term_clone = term.clone();
        let input_thread = thread::spawn(move || loop {
            if let Ok(key) = term_clone.poll_event() {
                if key == TermEvent::User(()) {
                    break;
                }

                let (key, action_chain) = input.translate_event(key);
                for event in action_chain.into_iter() {
                    let _ = tx_clone.send((key, event));
                }
            }
        });

        //------------------------------------------------------------------------------
        // reader

        let reader = Reader::with_options(options).source(source);

        //------------------------------------------------------------------------------
        // model + previewer
        let mut model = Model::new(rx, tx, reader, term.clone(), options);
        let ret = model.start();
        let _ = term.send_event(TermEvent::User(())); // interrupt the input thread
        let _ = input_thread.join();
        ret
    }

    // 10 -> TermHeight::Fixed(10)
    // 10% -> TermHeight::Percent(10)
    fn parse_height_string(string: &str) -> TermHeight {
        if string.ends_with('%') {
            TermHeight::Percent(string[0..string.len() - 1].parse().unwrap_or(100))
        } else {
            TermHeight::Fixed(string.parse().unwrap_or(0))
        }
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait FuzzyPassItem: AsAny + Send + Sync + 'static {
    /// The string to be used for matching (without color)
    fn text(&self) -> Cow<str>;

    /// The content to be displayed on the item list, could contain ANSI properties
    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::from(context)
    }

    /// Custom preview content, default to `ItemPreview::Global` which will use global preview
    /// setting(i.e. the command set by `preview` option)
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Global
    }

    /// Get output text(after accept), default to `text()`
    /// Note that this function is intended to be used by the caller of skim and will not be used by
    /// skim. And since skim will return the item back in `SkimOutput`, if string is not what you
    /// want, you could still use `downcast` to retain the pointer to the original struct.
    fn output(&self) -> Cow<str> {
        self.text()
    }

    /// we could limit the matching ranges of the `get_text` of the item.
    /// providing (start_byte, end_byte) of the range
    fn get_matching_ranges(&self) -> Option<&[(usize, usize)]> {
        None
    }
}

pub fn run(app: App) -> Result<()> {
    let app = Arc::new(Mutex::new(app));
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    print!("{}", CLEAR_SCREEN);
    print!("{}", HIDE_CURSOR);

    stdout.flush().unwrap();

    let (tx, rx) = mpsc::channel::<String>();

    let app2 = Arc::clone(&app);
    let input_thread = thread::spawn(move || handle_query(tx, Arc::clone(&app)));
    let display_thread = thread::spawn(move || handle_display(rx, app2));

    input_thread.join().unwrap();
    display_thread.join().unwrap();

    print!("{}", CLEAR_SCREEN);
    print!("Exiting...");
    io::stdout().flush().unwrap();
    stdout.suspend_raw_mode().unwrap();
    Ok(())
}
