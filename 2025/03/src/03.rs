aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let bat1 = line[..line.len() - 1]
                .char_indices()
                .fold(
                    (0, '0'),
                    |(pos, ch), (i, c)| if c > ch { (i, c) } else { (pos, ch) },
                );
            let bat2 = line[bat1.0 + 1..]
                .chars()
                .fold('0', |ch, c| if c > ch { c } else { ch });
            let joltage = bat1.1.to_digit(10).unwrap() * 10 + bat2.to_digit(10).unwrap();
            joltage
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let mut joltage = 0u64;
            let mut next_start_index = 0;
            let mut missing_digits = 12;
            while missing_digits > 0 {
                let d = line[next_start_index..=line.len() - missing_digits]
                    .char_indices()
                    .fold(
                        (0, '0'),
                        |(pos, ch), (i, c)| {
                            if c > ch {
                                (i, c)
                            } else {
                                (pos, ch)
                            }
                        },
                    );
                joltage = joltage * 10 + d.1.to_digit(10).unwrap() as u64;
                next_start_index += d.0 + 1;
                missing_digits -= 1;
            }
            joltage
        })
        .sum::<u64>()
}
