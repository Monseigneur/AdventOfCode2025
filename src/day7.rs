use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

const DAY: usize = 7;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn below(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left_down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col - 1,
        }
    }

    fn right_down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col + 1,
        }
    }
}

fn part_1(contents: &str) -> usize {
    let (grid, start) = parse_manifold(contents);

    simulate_tachyons(&grid, &start)
}

fn parse_manifold(contents: &str) -> (Grid, Point) {
    let mut start = None;

    let grid: Grid = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for (row, row_data) in grid.iter().enumerate() {
        for (col, c) in row_data.iter().enumerate() {
            if c == &'S' {
                start = Some(Point::new(row, col));
            }
        }
    }

    (grid, start.unwrap())
}

fn simulate_tachyons(grid: &Grid, start: &Point) -> usize {
    let mut beams = VecDeque::new();
    let mut visited = HashSet::new();

    beams.push_back(*start);

    let mut splits = 0;
    while let Some(beam_point) = beams.pop_front() {
        if !visited.insert(beam_point) {
            continue;
        }

        let location = grid[beam_point.row][beam_point.col];

        if location == '^' {
            splits += 1;
            let adjacent = get_adjacent_points(&beam_point, grid);

            for point in adjacent {
                beams.push_back(point);
            }
        } else if beam_point.row < grid.len() - 1 {
            beams.push_back(beam_point.below());
        }
    }

    splits
}

fn get_adjacent_points(point: &Point, grid: &Grid) -> Vec<Point> {
    let mut adjacent = vec![];

    if point.col > 0 {
        adjacent.push(point.left_down());
    }

    if point.col < grid[0].len() - 1 {
        adjacent.push(point.right_down());
    }

    adjacent
}

fn part_2(contents: &str) -> usize {
    let (grid, start) = parse_manifold(contents);

    simulate_tachyons_v2(&grid, &start)
}

fn simulate_tachyons_v2(grid: &Grid, start: &Point) -> usize {
    let mut beams = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut dist: HashMap<Point, usize> = HashMap::new();

    // Since the beams never go up, processing a row will give all beams that can reach the next row.
    // Aggregate the count of beams that would reach a point in the next row, carrying through splitters.

    beams.push(Reverse((start.row, *start)));
    dist.insert(*start, 1);

    let mut reached_edge = 0;
    while let Some(Reverse((_, beam_point))) = beams.pop() {
        if !visited.insert(beam_point) {
            continue;
        }

        let beam_count = *dist.get(&beam_point).unwrap();

        let next_beams = if grid[beam_point.row][beam_point.col] == '^' {
            vec![beam_point.left_down(), beam_point.right_down()]
        } else if beam_point.row < grid.len() - 1 {
            vec![beam_point.below()]
        } else {
            reached_edge += beam_count;

            vec![]
        };

        for next_beam in next_beams {
            dist.entry(next_beam)
                .and_modify(|c| *c += beam_count)
                .or_insert(beam_count);
            beams.push(Reverse((next_beam.row, next_beam)));
        }
    }

    reached_edge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 21);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1662);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 40);
    }

    #[test]
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 10);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        // 11:10pm (1162 - 1) * 2 isn't right lol
        assert_eq!(part_2(&contents), 40941112789504);
    }
}
