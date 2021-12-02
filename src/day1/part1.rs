use super::*;

pub fn run() -> anyhow::Result<u64> {
    let lines = data()?;

    let ret = lines.windows(2).fold(0, |acc, next| if next[1] > next[0] { acc + 1 } else { acc } );

    Ok(ret)
}
