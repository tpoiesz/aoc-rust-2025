use std::ops::Range;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let split_input = input.trim().split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(split_input.0);
    let merged_ranges = merge_ranges(&mut ranges);
    let mut ids = split_input.1.lines().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<_>>();
    ids.sort_unstable();

    // The following works by simulating inserting the start and end of a range into the listed sort of ids
    // If binary_search finds the value it returns the index, otherwise it returns the index where the value should be inserted to keep the list sorted
    // By comparing the difference between where the start and end of the range would be inserted, we know how many elements fit into the range
    let position = |x| ids.binary_search(&x).unwrap_or_else(|e| e);
    let count: usize = merged_ranges.iter().map(|range| position(range.end) - position(range.start)).sum();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let split_input = input.trim().split_once("\n\n").unwrap();
    let mut ranges = parse_ranges(split_input.0);

    let merged_ranges = merge_ranges(&mut ranges);

    let result = merged_ranges.iter().map(|range| range.end - range.start).sum();

    Some(result)
}

fn parse_ranges(input: &str) -> Vec<Range<u64>> {
    input.lines().map(|line| {
        let range = line.split_once('-').unwrap();
        range.0.parse::<u64>().unwrap()..range.1.parse::<u64>().unwrap()
    }).collect::<Vec<_>>()
}

fn merge_ranges(ranges: &mut Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_unstable_by_key(|range| range.start);

    let mut merged_ranges = Vec::new();
    let mut merged_range = 0..0;

    for range in ranges {
        if range.start <= merged_range.end {
            merged_range.end = merged_range.end.max(range.end + 1)
        } else {
            merged_ranges.push(merged_range);
            merged_range = range.start..range.end+1;
        }
    }
    merged_ranges.push(merged_range);
    merged_ranges
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
