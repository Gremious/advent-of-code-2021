use super::*;

pub mod part1 {
    pub fn run() -> anyhow::Result<u64> {
        let lines = super::data()?;
        let half_length = lines.len() as u64 / 2;

        let mut gamma_rate: [u8; 12] = Default::default();

        let bytes_vec: Vec<Vec<u8>> = lines.into_iter()
            .map(|x| x.chars()
                .map(|x| x.to_digit(2).unwrap() as u8).collect()
            )
            .collect();

        (0..12)
            .map(|i| bytes_vec.iter()
                .map(|bytes| bytes[i] as u64).sum::<u64>()
            )
            .enumerate()
            .for_each(|(i, x)| if x > half_length { gamma_rate[i] = 1 } else { gamma_rate[i] = 0; });

        let epsilon_rate: String = gamma_rate.clone().into_iter().map(|x| ((x == 0) as u8).to_string()).collect();
        let epsilon_rate = u64::from_str_radix(&epsilon_rate, 2)?;

        let gamma_rate: String = gamma_rate.into_iter().map(|x| x.to_string()).collect();
        let gamma_rate = u64::from_str_radix(&gamma_rate, 2)?;

        assert_eq!(gamma_rate * epsilon_rate, 3923414);
        Ok(gamma_rate * epsilon_rate)
    }
}

fn data() -> anyhow::Result<Vec<String>> {
    Ok(BufReader::new(File::open("./src/resources/3")?).lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>())
}
