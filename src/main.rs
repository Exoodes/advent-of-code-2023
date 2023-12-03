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

pub fn read_file(day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("input").join(format!("day_{:0>2}.txt", day));
    let f = fs::read_to_string(&filepath);
    f.expect(&format!("could not open input file {}", filepath.display()))
}

fn main() -> Result<()> {
    let args = Args::parse();

    let solution = match args.day {
        1 => solutions::day_01::solve(&read_file(args.day)),
        2 => solutions::day_02::solve(&read_file(args.day)),
        3 => solutions::day_03::solve(&read_file(args.day)),
        _ => return Err(anyhow!("Invalid day: {}", args.day)),
    };

    print!("{}", solution);
    Ok(())
}
