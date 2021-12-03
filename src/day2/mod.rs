use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod part1;
pub mod part2;

fn data() -> anyhow::Result<Vec<i32>> {
    Ok(BufReader::new(File::open("./src/resources/1")?).lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>())
}
