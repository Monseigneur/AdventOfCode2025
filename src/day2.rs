use std::{collections::HashSet, ops::RangeInclusive};

const DAY: usize = 2;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let ranges = parse_ranges(contents);

    ranges.into_iter().map(|range| find_invalid_ids(&range)).sum()
}

fn parse_ranges(contents: &str) -> Vec<RangeInclusive<usize>> {
    contents.split(",").map(|s| {
        let pieces = s.split("-").map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();

        pieces[0]..=pieces[1]
    }).collect()
}

fn find_invalid_ids(range: &RangeInclusive<usize>) -> usize {
    let start = split_number(*range.start(), true);
    let end = split_number(*range.end(), false);

    let mut invalid_sum = 0;
    for prefix in start..=end {
        let val = make_invalid_id(prefix, 2);

        if range.contains(&val) {
            invalid_sum += val;
        }
    }

    invalid_sum
}

fn split_number(val: usize, is_start: bool) -> usize {
    let digits = val.to_string();
    let len = digits.len();

    if is_start {
        if len % 2 == 0 {
            // Can use the top half (or bigger)
            let prefix = digits[0..(len / 2)].parse::<usize>().unwrap();

            let result = make_invalid_id(prefix, 2);

            if result < val {
                prefix + 1
            } else {
                prefix
            }
        } else {
            let prefix = 10usize.pow(len as u32 / 2);

            prefix
        }
    } else {
        if len % 2 == 0 {
            // Can use the top half (or smaller)
            let prefix = digits[0..(len / 2)].parse::<usize>().unwrap();

            let result = make_invalid_id(prefix, 2);

            if result > val {
                prefix - 1
            } else {
                prefix
            }
        } else {
            let prefix = 10usize.pow(len as u32 / 2) - 1;
            prefix
        }
    }
}

fn part_2(contents: &str) -> usize {
    let ranges = parse_ranges(contents);

    ranges.into_iter().map(|range| find_invalid_ids_v2(&range)).sum()
}

fn find_invalid_ids_v2(range: &RangeInclusive<usize>) -> usize {
    let start_digits = range.start().ilog10() + 1;
    let end_digits = range.end().ilog10() + 1;

    let max_prefix_digits = start_digits.max(end_digits) / 2;

    let mut invalid_ids = HashSet::new();

    for prefix_size in 1..=max_prefix_digits {
        let min_scale = (start_digits + prefix_size - 1) / prefix_size;
        let max_scale = end_digits / prefix_size;

        for scale in min_scale..=max_scale {
            if scale == 1 {
                continue;
            }

            for prefix in 10usize.pow(prefix_size - 1)..=(10usize.pow(prefix_size) - 1) {
                let val = make_invalid_id(prefix, scale as usize);

                if range.contains(&val) {
                    invalid_ids.insert(val);
                }
            }
        }
    }

    invalid_ids.into_iter().sum()
}

fn make_invalid_id(prefix: usize, scale: usize) -> usize {
    let digits = prefix.to_string();

    let result = digits.repeat(scale);

    result.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 1227775554);
    }

    #[test]
    fn test_example2_part_1() {
        let contents = "1-19";

        assert_eq!(part_1(&contents), 11);
    }


    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 23560874270);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 4174379265);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 44143124633);
    }
}
