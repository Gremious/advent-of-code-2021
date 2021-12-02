use super::*;

pub fn run() -> anyhow::Result<u64> {
    let file = File::open("./src/resources/1")?;
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let ret = lines
        .windows(2).fold(0, |acc, next| if next[1] > next[0] { acc + 1 } else { acc } );

    Ok(ret)
}
