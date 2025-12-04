const DAY: usize = 4;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;

fn part_1(contents: &str) -> usize {
    let grid = parse_grid(contents);

    count_rolls(grid)
}

fn parse_grid(contents: &str) -> Grid {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_rolls(grid: Grid) -> usize {
    let mut free_rolls = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '@' {
                continue;
            }

            if count_surrounding_rolls(&grid, r, c) < 4 {
                free_rolls += 1;
            }
        }
    }

    free_rolls
}

fn count_surrounding_rolls(grid: &Grid, row: usize, col: usize) -> usize {
    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;

    let mut surrounding_rolls = 0;
    for r in row.saturating_sub(1)..=max_row.min(row + 1) {
        for c in col.saturating_sub(1)..=max_col.min(col + 1) {
            if r == row && c == col {
                continue;
            }

            if grid[r][c] == '@' {
                surrounding_rolls += 1;
            }
        }
    }

    surrounding_rolls
}

fn part_2(contents: &str) -> usize {
    let mut grid = parse_grid(contents);

    let mut free_rolls = 0;

    while let Some(removed_rolls) = count_and_remove_rolls(&mut grid) {
        free_rolls += removed_rolls;
    }

    free_rolls
}

fn count_and_remove_rolls(grid: &mut Grid) -> Option<usize> {
    let mut free_rolls = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] != '@' {
                continue;
            }

            if count_surrounding_rolls(&grid, r, c) < 4 {
                grid[r][c] = '.';
                free_rolls += 1;
            }
        }
    }

    (free_rolls != 0).then_some(free_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 13);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1372);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 43);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 7922);
    }
}
