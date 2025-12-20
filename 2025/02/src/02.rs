use aoc::{IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    fn is_invalid_id(id: &str) -> bool {
        id.len() % 2 == 0 && id[..id.len() / 2] == id[id.len() / 2..]
    }

    let line = input.lines().next_uw();
    line.split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse_uw::<u64>()..=end.parse_uw())
                .filter(|id| is_invalid_id(&format!("{id}")))
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    fn is_invalid_id(id: &str) -> bool {
        (1..=id.len() / 2)
            .filter(|sublen| id.len() % sublen == 0)
            .any(|sublen| id.split(&id[..sublen]).all(|p| p.is_empty()))
    }

    let line = input.lines().next_uw();
    line.split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse_uw::<u64>()..=end.parse_uw())
                .filter(|id| is_invalid_id(&format!("{id}")))
                .sum::<u64>()
        })
        .sum::<u64>()
}
