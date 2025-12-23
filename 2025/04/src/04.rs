use std::collections::HashSet;

aoc::parts!(1, 2);

type Position = (usize, usize);

struct PositonNeighboorIterator {
    pos: Position,
    current_pos: Position,
}
impl PositonNeighboorIterator {
    fn new(pos: Position) -> Self {
        Self {
            pos,
            current_pos: (pos.0.saturating_sub(1), pos.1.saturating_sub(1)),
        }
    }
}
impl Iterator for PositonNeighboorIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos.1 > self.pos.1 + 1 {
            return None;
        }
        let next_position = self.current_pos;
        self.current_pos.0 += 1;
        if self.current_pos.0 > self.pos.0 + 1 {
            self.current_pos.0 = self.pos.0.saturating_sub(1);
            self.current_pos.1 += 1;
        }

        if self.pos == next_position {
            self.next()
        } else {
            Some(next_position)
        }
    }
}

fn neighboors(pos: Position) -> impl Iterator<Item = Position> {
    PositonNeighboorIterator::new(pos)
}

#[derive(Debug)]
struct Grid(HashSet<Position>);
impl Grid {
    fn new<'a>(lines: impl IntoIterator<Item = &'a str>) -> Self {
        Self(
            lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.char_indices()
                        .filter_map(move |(x, c)| (c == '@').then_some((x, y)))
                })
                .collect(),
        )
    }

    fn neighboor_count(&self, pos: Position) -> usize {
        neighboors(pos)
            .filter(|pos| self.0.get(pos).is_some())
            .count()
    }

    fn get_occupied_positions<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        self.0.iter().copied()
    }

    fn remove_positions(&mut self, positions: impl IntoIterator<Item = Position>) -> usize {
        positions
            .into_iter()
            .scan((), |_, pos| Some(self.0.remove(&pos)))
            .count()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let grid = Grid::new(input);
    grid.get_occupied_positions()
        .filter(|pos| grid.neighboor_count(*pos) < 4)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut grid = Grid::new(input);
    let mut total_count = 0;
    loop {
        let positions = grid
            .get_occupied_positions()
            .filter(|pos| grid.neighboor_count(*pos) < 4)
            .collect::<Vec<_>>();
        let removed_count = grid.remove_positions(positions);
        total_count += removed_count;
        if removed_count == 0 {
            return total_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighboors_top_left() {
        assert_eq!(
            neighboors((0, 0)).collect::<Vec<_>>(),
            [(1, 0), (0, 1), (1, 1)]
        );
    }

    #[test]
    fn neighboors_top() {
        assert_eq!(
            neighboors((2, 0)).collect::<Vec<_>>(),
            [(1, 0), (3, 0), (1, 1), (2, 1), (3, 1)]
        );
    }

    #[test]
    fn neighboors_left() {
        assert_eq!(
            neighboors((0, 2)).collect::<Vec<_>>(),
            [(0, 1), (1, 1), (1, 2), (0, 3), (1, 3)]
        );
    }

    #[test]
    fn neighboors_middle() {
        assert_eq!(
            neighboors((2, 2)).collect::<Vec<_>>(),
            [
                (1, 1),
                (2, 1),
                (3, 1),
                (1, 2),
                (3, 2),
                (1, 3),
                (2, 3),
                (3, 3)
            ]
        );
    }
}
