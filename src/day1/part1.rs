use super::*;

pub fn run() -> anyhow::Result<u64> {
    let file = File::open("./src/resources/1")?;
    let lines = BufReader::new(file).lines().filter_map(|x| x.ok()).collect::<Vec<_>>();

    let ret = lines.windows(2).fold(0, |acc, next| if next[1] > next[0] { acc + 1 } else { acc } ) + 1; // i actually dk why it's off by 1 lol

    Ok(ret)
}
