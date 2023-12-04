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
        4 => solutions::day_04::solve(&read_file(args.day)),
        5 => solutions::day_05::solve(&read_file(args.day)),
        6 => solutions::day_06::solve(&read_file(args.day)),
        7 => solutions::day_07::solve(&read_file(args.day)),
        8 => solutions::day_08::solve(&read_file(args.day)),
        9 => solutions::day_09::solve(&read_file(args.day)),
        10 => solutions::day_10::solve(&read_file(args.day)),
        11 => solutions::day_11::solve(&read_file(args.day)),
        12 => solutions::day_12::solve(&read_file(args.day)),
        13 => solutions::day_13::solve(&read_file(args.day)),
        14 => solutions::day_14::solve(&read_file(args.day)),
        15 => solutions::day_15::solve(&read_file(args.day)),
        16 => solutions::day_16::solve(&read_file(args.day)),
        17 => solutions::day_17::solve(&read_file(args.day)),
        18 => solutions::day_18::solve(&read_file(args.day)),
        19 => solutions::day_19::solve(&read_file(args.day)),
        20 => solutions::day_20::solve(&read_file(args.day)),
        21 => solutions::day_21::solve(&read_file(args.day)),
        22 => solutions::day_22::solve(&read_file(args.day)),
        23 => solutions::day_23::solve(&read_file(args.day)),
        24 => solutions::day_24::solve(&read_file(args.day)),
        _ => return Err(anyhow!("Invalid day: {}", args.day)),
    };

    print!("{}", solution);
    Ok(())
}
