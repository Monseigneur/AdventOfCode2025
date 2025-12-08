use std::{cmp::Reverse, collections::HashSet};

const DAY: usize = 8;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(text: &str) -> Self {
        let pieces = text
            .split(",")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        Self {
            x: pieces[0],
            y: pieces[1],
            z: pieces[2],
        }
    }

    fn distance_sq(&self, other: &Point) -> isize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn part_1(contents: &str) -> usize {
    let junction_boxes = parse_junction_boxes(contents);

    join_junction_boxes(junction_boxes, 1000)
}

fn parse_junction_boxes(contents: &str) -> Vec<Point> {
    contents.lines().map(Point::new).collect()
}

fn join_junction_boxes(junction_boxes: Vec<Point>, num_to_join: usize) -> usize {
    let distances = sort_by_distance(&junction_boxes);

    let mut circuits: Vec<HashSet<Point>> = vec![];

    for (_, a, b) in distances.iter().take(num_to_join) {
        let a_pos = circuits.iter().position(|circuit| circuit.contains(a));
        let b_pos = circuits.iter().position(|circuit| circuit.contains(b));

        match (a_pos, b_pos) {
            // Neither seen, join into new circuit.
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(*a);
                circuit.insert(*b);
                circuits.push(circuit)
            }
            // One is seen, add other to seen set.
            (Some(i), None) => {
                circuits[i].insert(*b);
            }
            (None, Some(i)) => {
                circuits[i].insert(*a);
            }
            // Both are in different groups, need to join.
            (Some(i), Some(j)) if i != j => {
                if i < j {
                    let other_circuit = circuits.remove(j);
                    circuits[i].extend(other_circuit);
                } else {
                    let other_circuit = circuits.remove(i);
                    circuits[j].extend(other_circuit);
                }
            }
            _ => {}
        }
    }

    circuits.sort_by_key(|circuit| Reverse(circuit.len()));

    circuits
        .iter()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

fn sort_by_distance(junction_boxes: &[Point]) -> Vec<(isize, Point, Point)> {
    let mut distances = vec![];

    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let a = junction_boxes[i];
            let b = junction_boxes[j];

            distances.push((a.distance_sq(&b), a, b));
        }
    }

    distances.sort_by(|(a, ..), (b, ..)| a.cmp(b));

    distances
}

fn part_2(contents: &str) -> usize {
    let junction_boxes = parse_junction_boxes(contents);

    join_junction_boxes_v2(junction_boxes)
}

fn join_junction_boxes_v2(junction_boxes: Vec<Point>) -> usize {
    let distances = sort_by_distance(&junction_boxes);
    let mut join_score = None;

    let mut circuits: Vec<HashSet<Point>> = vec![];

    for (_, a, b) in distances.iter() {
        let a_pos = circuits.iter().position(|circuit| circuit.contains(a));
        let b_pos = circuits.iter().position(|circuit| circuit.contains(b));

        match (a_pos, b_pos) {
            // Neither seen, join into new circuit.
            (None, None) => {
                let mut circuit = HashSet::new();
                circuit.insert(*a);
                circuit.insert(*b);
                circuits.push(circuit)
            }
            // One is seen, add other to seen set.
            (Some(i), None) => {
                circuits[i].insert(*b);
            }
            (None, Some(i)) => {
                circuits[i].insert(*a);
            }
            // Both are in different groups, need to join.
            (Some(i), Some(j)) if i != j => {
                if i < j {
                    let other_circuit = circuits.remove(j);
                    circuits[i].extend(other_circuit);
                } else {
                    let other_circuit = circuits.remove(i);
                    circuits[j].extend(other_circuit);
                }
            }
            _ => {}
        }

        if circuits
            .first()
            .is_some_and(|circuit| circuit.len() == junction_boxes.len())
        {
            join_score = Some(a.x * b.x);
            break;
        }
    }

    join_score.unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        let junction_boxes = parse_junction_boxes(&contents);

        assert_eq!(join_junction_boxes(junction_boxes, 10), 40);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 79560);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 25272);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 31182420);
    }
}
