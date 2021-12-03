mod day1;
mod day2;

fn main() -> anyhow::Result<()> {
    println!("Day 1 Part 1: {}", day1::part1::run()?);
    println!("Day 1 Part 2: {}", day1::part2::run()?);
    println!("Day 2 Part 1: {}", day2::part1::run()?);
    println!("Day 2 Part 2: {}", day2::part2::run()?);
    Ok(())
}
