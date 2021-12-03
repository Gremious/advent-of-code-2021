pub fn run() -> anyhow::Result<u128> {
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
        .for_each(|(i, x)| { if x > half_length { gamma_rate[i] = 1 } else { gamma_rate[i] = 0; } });

    eprintln!("gamma_rate = {:?}", gamma_rate);

    let epsilon_rate: String = gamma_rate.clone().map(|x| ((x == 0) as u8).to_string()).into_iter().collect();

    eprintln!("epsilon_rate = {:?}", epsilon_rate);

    let epsilon_rate = u128::from_str_radix(&epsilon_rate, 2)?;
    eprintln!("epsilon_rate = {:?}", epsilon_rate);

    let gamma_rate: String = gamma_rate.clone().map(|x| x.to_string()).into_iter().collect();
    let gamma_rate = u128::from_str_radix(&gamma_rate, 2)?;
    eprintln!("gamma_rate = {:?}", gamma_rate);

    Ok(gamma_rate * epsilon_rate)
    // Ok(0)
}
