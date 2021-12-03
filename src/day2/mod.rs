use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod part1;
pub mod part2;

fn data() -> anyhow::Result<Vec<String>> {
    Ok(BufReader::new(File::open("./src/resources/2")?).lines()
        .map(|x| x?)
        .collect::<Vec<String>>())
}
