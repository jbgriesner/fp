use fp::app::App;
use fp::prelude::*;
use fp::run;
use inquire::error::InquireResult;
use inquire::Select;
use std::fmt::Display;
use std::fmt::Formatter;

use clap::{Parser, Subcommand};

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

fn main() -> InquireResult<()> {
    let args = Args::parse();
    let cmd: Command;

    if args.cmd.is_some() {
        cmd = args.cmd.unwrap();
    } else {
        cmd = Select::new("Command:", Command::VARIANTS.to_vec()).prompt()?;
    }

    match cmd {
        Command::Create => println!("create"),
        Command::Get => println!("get"),
    }
    Ok(())
    // let app = App::new();
    // run(app)
}
