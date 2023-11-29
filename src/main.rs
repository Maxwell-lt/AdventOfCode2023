use std::{
    fs,
    time::{Duration, Instant},
};

use clap::Parser;
use color_eyre::eyre::{bail, Result, Context};
use days::days::get_solver;

mod days;
mod solver;
mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    print_header();
    if let Some(day) = args.day {
        let input = read_input(day)?;
        if let Some(part) = args.part {
            run_and_print(day, part, &input)?;
        } else {
            run_and_print(day, 1, &input)?;
            run_and_print(day, 2, &input)?;
        }
    } else {
        for day in 1..26 {
            if let Ok(input) = read_input(day) {
                run_and_print(day, 1, &input)?;
                run_and_print(day, 2, &input)?;
            }
        }
    }
    Ok(())
}

fn read_input(day: u8) -> Result<String> {
    fs::read_to_string(format!("./input/Day{}", day)).wrap_err(format!("Failed to read input for day {}", day))
}

fn print_header() {
    println!("Advent of Code 2023");
    println!(
        "{: ^5}|{: ^6}|{: ^25}|{: ^10}",
        "Day", "Part", "Solution", "Time"
    );
    println!("{:_^49}", "");
}

fn run_and_print(day: u8, part: u8, input: &str) -> Result<()> {
    let answer = run(day, part, input)?;
    if let Some(answer) = answer {
        println!(
            "{: ^5}|{: ^6}|{: ^25}|{: ^10}",
            day,
            part,
            answer.0,
            format_time(answer.1)
        );
    }
    Ok(())
}

fn run(day: u8, part: u8, input: &str) -> Result<Option<(i64, Duration)>> {
    let solver = get_solver(day);
    match part {
        1 => Ok(time(|| solver.solve1(input))),
        2 => Ok(time(|| solver.solve2(input))),
        _ => bail!("Tried to run a part other than 1 or 2!"),
    }
}

fn format_time(d: Duration) -> String {
    format!("{}ms", d.as_millis())
}

fn time<F>(f: F) -> Option<(i64, Duration)>
where
    F: Fn() -> Option<i64>,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    result.map(|r| (r, duration))
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    part: Option<u8>,
}
