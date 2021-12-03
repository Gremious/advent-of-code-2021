use super::*;

pub fn run() -> anyhow::Result<u64> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    let lines = data()?;

    for line in lines {
        let (command, value) = line.split_once(' ').unwrap();
        let value = value.parse::<u64>()?;

        match command {
            "forward" => {
                horizontal_position += value;
                depth += aim * value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }

    Ok(horizontal_position * depth)
}
