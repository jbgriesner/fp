use fp::app::App;
use fp::prelude::*;
// use fp::run;
use inquire::error::InquireResult;
use inquire::Select;
use std::fmt::Display;
use std::fmt::Formatter;

use clap::{Parser, Subcommand};

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tuikit::prelude::*;

const COL: usize = 4;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand, Debug, Clone, Copy)]
enum Command {
    Create,
    Get,
}

impl Command {
    const VARIANTS: &'static [Command] = &[Self::Create, Self::Get];
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

fn run() -> InquireResult<()> {
    let args = Args::parse();
    let cmd: Command;

    if args.cmd.is_some() {
        cmd = args.cmd.unwrap();
    } else {
        cmd = Select::new("Select your command:", Command::VARIANTS.to_vec()).prompt()?;
    }

    match cmd {
        Command::Create => println!("create"),
        Command::Get => println!("get"),
    }
    Ok(())
}

fn main() -> fp::prelude::Result<()> {
    // run().map_err(|_| Error::Generic("Error occurred during execution".into()))?;
    // Ok(())

    // let app = App::new();
    // run(app)

    env_logger::init();
    let term = Arc::new(Term::with_height(TermHeight::Fixed(10)).unwrap());
    let _ = term.enable_mouse_support();
    let now = Instant::now();

    print_banner(&term);

    let th = thread::spawn(move || {
        while let Ok(ev) = term.poll_event() {
            match ev {
                Event::Key(Key::Ctrl('c')) => break,
                Event::Key(Key::Char('r')) => {
                    let term = term.clone();
                    thread::spawn(move || {
                        let _ = term.pause();
                        println!("restart in 2 seconds");
                        thread::sleep(Duration::from_secs(2));
                        let _ = term.restart();
                        let _ = term.clear();
                    });
                }
                _ => (),
            }

            print_banner(&term);
            print_event(&term, ev, &now);
        }
    });
    let _ = th.join();

    Ok(())
}

fn print_banner(term: &Term) {
    let (_, height) = term.term_size().unwrap_or((5, 5));
    for row in 0..height {
        let _ = term.print(row, 0, format!("{} ", row).as_str());
    }
    let attr = Attr {
        fg: Color::GREEN,
        effect: Effect::UNDERLINE,
        ..Attr::default()
    };
    let _ = term.print_with_attr(0, COL, "How to use: (q)uit, (r)estart", attr);
    let _ = term.present();
}

fn print_event(term: &Term, ev: Event, now: &Instant) {
    let elapsed = now.elapsed();
    let (_, height) = term.term_size().unwrap_or((5, 5));
    let _ = term.print(1, COL, format!("{:?}", ev).as_str());
    let _ = term.print(
        height - 1,
        COL,
        format!(
            "time elapsed since program start: {}s + {}ms",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        )
        .as_str(),
    );
    let _ = term.present();
}
