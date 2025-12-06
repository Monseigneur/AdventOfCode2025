const DAY: usize = 1;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl Rotation {
    fn new(text: &str) -> Self {
        let direction = text.chars().nth(0).unwrap();
        let distance = text[1..].parse::<usize>().unwrap();

        match direction {
            'L' => Self::Left(distance),
            'R' => Self::Right(distance),
            _ => unreachable!(),
        }
    }
}

fn part_1(contents: &str) -> usize {
    let rotations = parse_rotations(contents);

    calculate_password(rotations)
}

fn parse_rotations(contents: &str) -> Vec<Rotation> {
    contents.lines().map(Rotation::new).collect()
}

fn calculate_password(rotations: Vec<Rotation>) -> usize {
    let mut current: isize = 50;
    let mut at_zero = 0;

    for rotation in rotations {
        match rotation {
            Rotation::Left(dist) => current = (current - dist as isize) % 100,
            Rotation::Right(dist) => current = (current + dist as isize) % 100,
        }

        if current == 0 {
            at_zero += 1;
        }
    }

    at_zero
}

fn part_2(contents: &str) -> usize {
    let rotations = parse_rotations(contents);

    calculate_password_v2(rotations)
}

fn calculate_password_v2(rotations: Vec<Rotation>) -> usize {
    let mut current = 50;
    let mut at_zero = 0;

    for rotation in rotations {
        match rotation {
            Rotation::Left(dist) => {
                if current <= dist {
                    // Crossing zero, count any additional wraparounds, but make sure not to double count starting at 0.
                    let rem = dist - current;
                    at_zero += rem / 100;

                    if current != 0 {
                        at_zero += 1;
                    }
                }

                let result = (current as isize - dist as isize) % 100;

                current = if result < 0 { result + 100 } else { result } as usize
            }
            Rotation::Right(dist) => {
                if (100 - current) <= dist {
                    // Crossing zero, increment by 1 and for any additional wraparounds.
                    let rem = dist - (100 - current);
                    at_zero += 1 + rem / 100;
                }

                current = (current + dist) % 100;
            }
        };
    }

    at_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 3);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1097);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 6);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 7101);
    }
}
