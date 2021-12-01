use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;

fn main() -> anyhow::Result<()> {
    println!("{}", day1::part1::run()?);
    Ok(())
}
