const DAY: usize = 6;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from_str(text: &str) -> Self {
        Self::from_char(text.chars().next().unwrap())
    }

    fn from_char(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => unreachable!(),
        }
    }
}

type Problem = (Vec<usize>, Operator);

fn part_1(contents: &str) -> usize {
    let problems = parse_problems(contents);

    problems
        .into_iter()
        .map(|problem| solve_problem(&problem))
        .sum()
}

fn parse_problems(contents: &str) -> Vec<Problem> {
    let mut lines = contents.lines().rev();

    let mut problems: Vec<Problem> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(Operator::from_str)
        .map(|op| (vec![], op))
        .collect();

    for line in lines.rev() {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .enumerate()
            .for_each(|(i, val)| {
                problems[i].0.push(val);
            });
    }

    problems
}

fn solve_problem(problem: &Problem) -> usize {
    match problem.1 {
        Operator::Add => problem.0.iter().sum(),
        Operator::Multiply => problem.0.iter().product(),
    }
}

fn part_2(contents: &str) -> usize {
    let data = parse_data(contents);

    calculate_result(data)
}

fn parse_data(contents: &str) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn calculate_result(data: Vec<Vec<char>>) -> usize {
    let mut final_result = 0;

    // Transpose the data to make it easier.
    let num_cols = data.len();
    let num_rows = data[0].len();
    let op_col = num_cols - 1;

    let mut op = None;
    let mut result = None;

    for r in 0..num_rows {
        if op.is_none() {
            op = Some(Operator::from_char(data[op_col][r]));
        }

        let is_end = (0..num_cols).all(|c| data[c][r] == ' ');

        if is_end {
            final_result += result.unwrap();
            result = None;
            op = None;

            continue;
        }

        let current_val = data
            .iter()
            .take(op_col)
            .filter(|col| col[r] != ' ')
            .fold(0, |acc, col| {
                acc * 10 + col[r].to_digit(10).unwrap() as usize
            });

        if let Some(r) = result {
            match op {
                Some(Operator::Add) => result = Some(r + current_val),
                Some(Operator::Multiply) => result = Some(r * current_val),
                _ => unreachable!(),
            }
        } else {
            result = Some(current_val);
        }
    }

    final_result += result.unwrap();

    final_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 4277556);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 5595593539811);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 3263827);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 10153315705125);
    }
}
