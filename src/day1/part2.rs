pub fn run() -> anyhow::Result<u64> {
    let lines = super::data()?;

    // could use https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.chunks too if u wanted to skip a collect? idk how efficient it is
    let sums = lines.windows(3).map(|x| x.iter().sum()).collect::<Vec<i32>>();
    let ret = sums.windows(2).fold(0, |acc, next| if next[1] > next[0] { acc + 1 } else { acc } );

    Ok(ret)
}
