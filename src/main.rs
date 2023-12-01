use anyhow::{anyhow, Result};
use clap::Parser;
use std::env;
use std::fs;

mod solutions;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Day of the puzzle, between 1 and 25
    day: u8,
}

pub fn read_file(day: usize) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("input").join(format!("day_{day}.txt"));
    let f = fs::read_to_string(&filepath);
    f.expect(&format!("could not open input file {}", filepath.display()))
}

fn main() -> Result<()> {
    let args = Args::parse();

    let solution = match args.day {
        1 => solutions::day_01::solve(&read_file(1)),
        _ => return Err(anyhow!("Invalid day: {}", args.day)),
    };

    print!("{}", solution);
    Ok(())
}
