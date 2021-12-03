use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
mod day2;
mod day3;

fn main() -> anyhow::Result<()> {
    println!("Day 1 Part 1: {}", day1::part1::run()?);
    println!("Day 1 Part 2: {}", day1::part2::run()?);
    println!("Day 2 Part 1: {}", day2::part1::run()?);
    println!("Day 2 Part 2: {}", day2::part2::run()?);
    // println!("Day 3 Part 1: {}", day3::part1::run()?);
    // println!("Day 3 Part 2: {}", day3::part2::run()?);
    Ok(())
}
