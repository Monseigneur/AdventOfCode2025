const DAY: usize = 3;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let battery_banks = parse_banks(contents);

    battery_banks
        .into_iter()
        .map(|bank| calculate_joltage(&bank, 2))
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

fn part_2(contents: &str) -> usize {
    let battery_banks = parse_banks(contents);

    battery_banks
        .into_iter()
        .map(|bank| calculate_joltage(&bank, 12))
        .sum()
}

fn calculate_joltage(battery_bank: &Vec<usize>, num_batteries: usize) -> usize {
    let mut best_max = 0;

    calculate_joltage_helper(
        battery_bank,
        0,
        battery_bank.len() - (num_batteries - 1),
        num_batteries,
        0,
        &mut best_max,
    );

    best_max
}

fn calculate_joltage_helper(
    battery_bank: &Vec<usize>,
    start: usize,
    end: usize,
    remaining_batteries: usize,
    current_val: usize,
    best: &mut usize,
) {
    if remaining_batteries == 0 {
        if current_val > *best {
            *best = current_val;
        }

        return;
    }

    let max_battery = battery_bank[start..end].iter().max().unwrap();

    let current_val = current_val * 10 + max_battery;

    for i in start..end {
        if battery_bank[i] != *max_battery {
            continue;
        }

        calculate_joltage_helper(
            battery_bank,
            i + 1,
            end + 1,
            remaining_batteries - 1,
            current_val,
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
