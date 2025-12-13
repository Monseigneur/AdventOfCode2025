const DAY: usize = 7;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;

fn part_1(contents: &str) -> usize {
    let grid = parse_manifold(contents);

    let (splits, _) = simulate_tachyons(&grid);

    splits
}

fn parse_manifold(contents: &str) -> Grid {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part_2(contents: &str) -> usize {
    let grid = parse_manifold(contents);

    let (_, beam_count) = simulate_tachyons(&grid);

    beam_count
}

fn simulate_tachyons(grid: &Grid) -> (usize, usize) {
    let mut beams = vec![0; grid[0].len()];

    let mut splits = 0;
    for row_data in grid.iter() {
        for (col, c) in row_data.iter().enumerate() {
            if c == &'S' {
                beams[col] = 1;
            } else if c == &'^' && beams[col] != 0 {
                beams[col - 1] += beams[col];
                beams[col + 1] += beams[col];
                beams[col] = 0;

                splits += 1;
            }
        }
    }

    (splits, beams.into_iter().sum())
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

        assert_eq!(part_2(&contents), 40941112789504);
    }
}
