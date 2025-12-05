use std::cmp::max;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let split_input = input.trim().split_once("\n\n").unwrap();
    let ranges = parse_ranges(split_input.0);
    let numbers = split_input.1.lines().map(|line| line.parse::<u64>().unwrap());

    let count = numbers.filter(|&n| ranges.iter().any(|range| n >= range.min && n <= range.max)).count();

    Some(count as u64)
}

#[derive(Clone, Copy, Debug)]
struct Range {
    min: u64,
    max: u64,
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input.lines().map(|line| {
        let range = line.split_once('-').unwrap();
        Range { min: range.0.parse::<u64>().unwrap(), max: range.1.parse::<u64>().unwrap() }
    }).collect::<Vec<_>>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let split_input = input.trim().split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(split_input.0);
    ranges.sort_by(|a, b| a.min.cmp(&b.min));

    let mut result = 0;

    let mut first_index = 0;
    while first_index < ranges.len() {
        let first = ranges[first_index];
        let mut maximum = first.max;
        let mut next_index = first_index + 1;
        loop {
            if next_index == ranges.len() { break; }
            let next = ranges[next_index];
            if next.min > maximum { break; }
            maximum = max(next.max, maximum);
            next_index = next_index + 1;
        }
        first_index = next_index;
        result += maximum - first.min + 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
