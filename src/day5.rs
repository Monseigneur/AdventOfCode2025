use std::ops::RangeInclusive;

const DAY: usize = 5;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let (ranges, ingredients) = parse_database(contents);

    let ranges = merge_ranges(ranges);

    count_spoiled_ingredient(&ranges, &ingredients)
}

fn parse_database(contents: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    let mut on_ranges = true;
    for line in contents.lines() {
        if line.is_empty() {
            on_ranges = false;
            continue;
        }

        if on_ranges {
            let range_pieces = line
                .split("-")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            ranges.push(range_pieces[0]..=range_pieces[1]);
        } else {
            let ingredient = line.parse::<usize>().unwrap();

            ingredients.push(ingredient);
        }
    }

    (ranges, ingredients)
}

fn count_spoiled_ingredient(ranges: &[RangeInclusive<usize>], ingredients: &[usize]) -> usize {
    ingredients
        .iter()
        .filter_map(|ingredient| {
            let result = ranges.binary_search_by(|range| {
                if range.contains(ingredient) {
                    std::cmp::Ordering::Equal
                } else {
                    range.start().cmp(ingredient)
                }
            });

            result.ok()
        })
        .count()
}

fn part_2(contents: &str) -> usize {
    let (ranges, _) = parse_database(contents);

    let ranges = merge_ranges(ranges);

    ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn merge_ranges(ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut ranges = ranges;
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges = vec![];
    let (mut current_start, mut current_end) = ranges[0].clone().into_inner();

    ranges.iter().skip(1).for_each(|range| {
        if (current_start..=current_end).contains(range.start()) {
            // Range overlaps, extend current range.
            current_end = *range.end().max(&current_end);
        } else {
            // Distinct range, can add current one to final list.
            merged_ranges.push(current_start..=current_end);

            (current_start, current_end) = range.clone().into_inner();
        }
    });

    merged_ranges.push(current_start..=current_end);

    merged_ranges
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

        assert_eq!(part_1(&contents), 733);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 14);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 345821388687084);
    }
}
