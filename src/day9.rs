const DAY: usize = 9;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    row: usize,
    col: usize,
}

impl Tile {
    fn new(text: &str) -> Self {
        let pieces = text
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            row: pieces[0],
            col: pieces[1],
        }
    }

    fn area(&self, other: &Tile) -> usize {
        (self.row.abs_diff(other.row) + 1) * (self.col.abs_diff(other.col) + 1)
    }
}

fn part_1(contents: &str) -> usize {
    let tiles = parse_tiles(contents);

    find_largest_area(tiles)
}

fn parse_tiles(contents: &str) -> Vec<Tile> {
    contents.lines().map(Tile::new).collect()
}

fn find_largest_area(tiles: Vec<Tile>) -> usize {
    tiles
        .iter()
        .enumerate()
        .map(|(i, first)| {
            tiles
                .iter()
                .skip(i)
                .map(|second| first.area(second))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn part_2(contents: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 50);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 4715966250);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 0);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 0);
    }
}
