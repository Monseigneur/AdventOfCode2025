use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

const DAY: usize = 10;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
    joltage: Vec<usize>,
}

impl Machine {
    fn new(text: &str) -> Self {
        let pieces = text.split_ascii_whitespace().collect::<Vec<_>>();

        let lights = pieces[0]
            .chars()
            .rev()
            .skip(1)
            .take(pieces[0].len() - 2)
            .fold(0, |acc, c| (acc << 1) + if c == '#' { 1 } else { 0 });

        let buttons = pieces
            .iter()
            .skip(1)
            .take(pieces.len() - 2)
            .map(|s| Machine::parse_group(s))
            .collect();

        let joltage = pieces[pieces.len() - 1];
        let joltage = joltage[1..joltage.len() - 1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Self {
            lights,
            buttons,
            joltage,
        }
    }

    fn parse_group(text: &str) -> usize {
        let len = text.len();

        text[1..(len - 1)]
            .split(",")
            .map(|s| 1 << s.parse::<usize>().unwrap())
            .sum()
    }
}

fn part_1(contents: &str) -> usize {
    let machines = parse_machines(contents);

    machines.into_iter().map(calculate_presses).sum()
}

fn parse_machines(contents: &str) -> Vec<Machine> {
    contents.lines().map(Machine::new).collect()
}

fn calculate_presses(machine: Machine) -> usize {
    let mut queue = BinaryHeap::new();
    let mut presses = HashMap::new();

    queue.push(Reverse((0, 0)));

    while let Some(Reverse((num_pushes, state))) = queue.pop() {
        if state == machine.lights {
            return num_pushes;
        }

        if let Some(best_count) = presses.get(&state)
            && *best_count <= num_pushes
        {
            continue;
        }

        presses.insert(state, num_pushes);

        for button in &machine.buttons {
            queue.push(Reverse((num_pushes + 1, state ^ button)));
        }
    }

    unreachable!()
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

        assert_eq!(part_1(&contents), 7);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 486);
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
