#[macro_use]
extern crate clap;

use std::error::Error;
use clap::{IntoApp, FromArgMatches};

mod cli;

fn main() -> Result<(), Box<Error>> {
    // FIXME: This should be `cli::Head::parse()`
    let args = cli::Head::from_argmatches(&cli::Head::into_app().get_matches());
    std::fs::read_to_string(&args.file)?
        .lines()
        .take(args.count)
        .for_each(|line| println!("{}", line));
    Ok(())
}
