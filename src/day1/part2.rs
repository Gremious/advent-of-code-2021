use super::*;

pub fn run() -> anyhow::Result<u64> {
    let file = File::open("./src/resources/1")?;
    let lines = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // could use https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.chunks too if u wanted to skip a collect? idk how efficient it is
    let sums = lines.windows(3).map(|x| x.into_iter().sum()).collect::<Vec<i32>>();
    let ret = sums.windows(2).fold(0, |acc, next| if next[1] > next[0] { acc + 1 } else { acc } );

    Ok(ret)
}

