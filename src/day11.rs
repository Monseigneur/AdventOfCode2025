use std::collections::HashMap;

const DAY: usize = 11;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type ServerRack = HashMap<String, Vec<String>>;

fn part_1(contents: &str) -> usize {
    let server_rack = parse_server_rack(contents);

    count_paths(&server_rack, &"you".to_string())
}

fn parse_server_rack(contents: &str) -> ServerRack {
    let mut server_rack: ServerRack = HashMap::new();

    contents.lines().for_each(|line| {
        let mut pieces = line.split(":");
        let server = pieces.next().unwrap();

        let targets = pieces
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .filter(|text| !text.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        server_rack
            .entry(server.to_string())
            .and_modify(|v| v.extend(targets.clone()))
            .or_insert(targets);
    });

    server_rack
}

fn count_paths(server_rack: &ServerRack, current: &String) -> usize {
    if current == "out" {
        return 1;
    }

    server_rack
        .get(current)
        .unwrap()
        .iter()
        .map(|destination| count_paths(server_rack, destination))
        .sum()
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

        assert_eq!(part_1(&contents), 5);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 574);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 0);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 0);
    }
}
