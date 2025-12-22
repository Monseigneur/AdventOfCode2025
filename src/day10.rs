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
    joltages: Vec<usize>,
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
        let joltages = joltage[1..joltage.len() - 1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Self {
            lights,
            buttons,
            joltages,
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

        if let Some(best_count) = presses.get(&state) && *best_count <= num_pushes {
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
    let machines = parse_machines(contents);

    machines.into_iter()
        .map(calculate_presses_joltage_v3)
        .sum()
}

fn calculate_presses_joltage_v3(machine: Machine) -> usize {
    // From the subreddit: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

    let button_cache = build_button_cache(&machine.buttons);
    let mut cache = HashMap::new();
    calculate_presses_joltage_helper(&button_cache, &machine.joltages, &mut cache).unwrap()
}

fn build_lights(joltages: &Vec<usize>) -> usize {
    joltages.iter().rev().fold(0, |acc, x| (acc << 1) + (x % 2))
}

fn build_button_cache(buttons: &Vec<usize>) -> HashMap<usize, Vec<Vec<usize>>> {
    let mut button_cache: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();

    for i in 0..=((1 << buttons.len()) - 1) {
        let mut mask = i;
        let mut index = 0;
        let mut button_group = vec![];
        while mask != 0 {
            if mask & 1 != 0 {
                button_group.push(buttons[index]);
            }

            mask >>= 1;
            index += 1;
        }

        let target = button_group.iter().fold(0, |state, x| state ^ x);

        button_cache.entry(target).or_default().push(button_group);
    }

    button_cache
}

fn apply_buttons(buttons: &Vec<usize>, joltages: &Vec<usize>) -> Option<Vec<usize>> {
    let mut joltages = joltages.clone();

    for button in buttons {
        let mut mask = *button;
        let mut index = 0;
        while mask != 0 {
            if mask & 1 != 0 {
                if joltages[index] == 0 {
                    return None;
                }

                joltages[index] -= 1;
            }

            mask >>= 1;
            index += 1;
        }
    }

    joltages.iter_mut().for_each(|x| *x /= 2);

    Some(joltages)
}

fn calculate_presses_joltage_helper(button_cache: &HashMap<usize, Vec<Vec<usize>>>, joltages: &Vec<usize>, cache: &mut HashMap<Vec<usize>, Option<usize>>) -> Option<usize>
{
    if joltages.iter().all(|x| *x == 0) {
        return Some(0);
    }

    if let Some(best) = cache.get(joltages) {
        return *best;
    }

    let target = build_lights(joltages);

    let mut best = None;
    if let Some(possible_buttons) = button_cache.get(&target) {
        for button_group in possible_buttons {
            let child_best = apply_buttons(button_group, joltages)
                .and_then(|new_joltages| calculate_presses_joltage_helper(button_cache, &new_joltages, cache))
                .and_then(|val| Some(val * 2 + button_group.len()));

            if best.is_none_or(|best| child_best.is_some_and(|child_best| child_best < best)) {
                best = child_best;
            }
        }
    }

    cache.insert(joltages.clone(), best);

    best
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

        assert_eq!(part_2(&contents), 33);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 17820);
    }
}
