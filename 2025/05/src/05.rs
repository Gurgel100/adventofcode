use std::{
    array::IntoIter,
    borrow::Borrow,
    collections::{BTreeSet, HashSet},
    ops::RangeInclusive,
};

use aoc::Parse;

aoc::parts!(1, 2);

#[derive(Clone, Eq, PartialEq)]
struct Range(RangeInclusive<u64>);
impl Range {
    const fn start(&self) -> &u64 {
        self.0.start()
    }

    const fn end(&self) -> &u64 {
        self.0.end()
    }

    fn combine(&self, other: &Self) -> Self {
        assert!(self.start() <= other.end());
        assert!(self.end() >= other.start());

        let new_start = *self.start().min(other.start());
        let new_end = *self.end().max(other.end());
        Self(new_start..=new_end)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }

    fn contains(&self, item: u64) -> bool {
        self.0.contains(&item)
    }
}
impl From<RangeInclusive<u64>> for Range {
    fn from(value: RangeInclusive<u64>) -> Self {
        Self(value)
    }
}
impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start().partial_cmp(&other.start())
    }
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(other.start())
    }
}
impl Borrow<u64> for Range {
    fn borrow(&self) -> &u64 {
        self.start()
    }
}
impl IntoIterator for Range {
    type Item = u64;
    type IntoIter = RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn parse_fresh_ranges<'a>(input: &mut (impl Iterator<Item = &'a str> + 'a)) -> BTreeSet<Range> {
    let mut ranges = BTreeSet::new();
    for line in input.take_while(|line| !line.is_empty()) {
        let (start, end) = line.split_once('-').unwrap();
        let mut range: Range = (start.parse_uw()..=end.parse_uw()).into();
        let ranges_to_delete = ranges
            .range(..=*range.end())
            .fold(vec![], |mut to_delete, r| {
                if range.overlaps(r) {
                    range = range.combine(r);
                    to_delete.push(r.clone());
                }
                to_delete
            });
        for r in ranges_to_delete {
            ranges.remove(&r);
        }

        ranges.insert(range);
    }
    ranges
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lines = input.lines();
    let fresh_ranges = parse_fresh_ranges(&mut lines);
    lines
        .filter(|line| fresh_ranges.iter().any(|r| r.contains(line.parse_uw())))
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let fresh_ranges = parse_fresh_ranges(&mut input.lines());
    fresh_ranges
        .into_iter()
        .flat_map(IntoIterator::into_iter)
        .count()
}
