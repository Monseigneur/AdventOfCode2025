const DAY: usize = 3;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let battery_banks = parse_banks(contents);

    battery_banks
        .into_iter()
        .map(|bank| calculate_joltage(&bank))
        .sum()
}

fn parse_banks(contents: &str) -> Vec<Vec<usize>> {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn calculate_joltage(battery_bank: &Vec<usize>) -> usize {
    let max_digit = battery_bank[0..(battery_bank.len() - 1)]
        .iter()
        .max()
        .unwrap();

    let mut current_max = 0;
    for i in 0..(battery_bank.len() - 1) {
        if battery_bank[i] != *max_digit {
            continue;
        }

        let second_max_digit = battery_bank[(i + 1)..].iter().max().unwrap();

        let max = max_digit * 10 + second_max_digit;

        if max > current_max {
            current_max = max;
        }
    }

    current_max
}

fn part_2(contents: &str) -> usize {
    let battery_banks = parse_banks(contents);

    battery_banks
        .into_iter()
        .map(|bank| calculate_joltage_v2(&bank))
        .sum()
}

fn calculate_joltage_v2(battery_bank: &Vec<usize>) -> usize {
    let mut best_max = 0;

    calculate_joltage_v2_helper(
        battery_bank,
        0,
        battery_bank.len() - 11,
        0,
        0,
        &mut best_max,
    );

    best_max
}

fn calculate_joltage_v2_helper(
    battery_bank: &Vec<usize>,
    start: usize,
    end: usize,
    current_digits: usize,
    current_val: usize,
    best: &mut usize,
) {
    if current_digits == 12 {
        if current_val > *best {
            *best = current_val;
        }

        return;
    }

    let max_digit = battery_bank[start..end].iter().max().unwrap();

    let current = current_val * 10 + max_digit;

    for i in start..end {
        if battery_bank[i] != *max_digit {
            continue;
        }

        calculate_joltage_v2_helper(
            battery_bank,
            i + 1,
            end + 1,
            current_digits + 1,
            current,
            best,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 357);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 17207);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 3121910778619);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 170997883706617);
    }
}
