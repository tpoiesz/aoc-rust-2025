use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.trim().split(',')
        .map(|line| line.split_once('-').unwrap())
        .collect::<Vec<_>>();

    let mut result : u64 = 0;

    for range in ranges {
        let (start_first, start_second) = split_number(range.0);
        let (end_first, end_second) = split_number(range.1);
        let start = if start_second > start_first { start_first + 1 } else { start_first };
        let end = if end_second < end_first { end_first - 1 } else { end_first };
        for num in start..=end {
            let num_size = num.ilog10() as usize + 1;
            let invalid = repeat_number(num, 2, num_size);
            result += invalid;
        }
    }

    Some(result)
}

// Splits up a number (as a string) into it's two halves and returns as a tuple of numbers
// If the number has an odd number of digits, it will be rounded up to the nearest factor of 10 and split up
// Ex 1. "1234" -> (12, 34)
// Ex 2. "123" (-> "1000") -> (10, 0)
fn split_number(num_string: &str) -> (u64, u64) {
    let size = num_string.len();
    if size % 2 == 0 {
        let halves = num_string.split_at(size / 2);
        (halves.0.parse::<u64>().unwrap(), halves.1.parse::<u64>().unwrap())
    } else {
        let base : u64 = 10;
        let exp = (size / 2) as u32;
        (base.pow(exp), 0)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.trim().split(',')
        .map(|line| {
            line.split_once('-').unwrap()
        })
        .collect::<Vec<_>>();

    let mut result : u64 = 0;

    for range in ranges {
        let start = range.0.parse::<u64>().unwrap();
        let end = range.1.parse::<u64>().unwrap();
        let start_str = range.0;
        let end_str = range.1;
        let start_len = start_str.len();
        let end_len = end_str.len();
        let max = end_str.split_at((end_len + 1)/2).0.parse::<u64>().unwrap();
        let mut checked = HashSet::new(); // We check '1' times 4 (=1111) but also '11' times 2 (=1111), use this set as a hacky way to prevent double counting
        for s in start_len..=end_len {
            for i in 1..=max {
                let size = i.ilog10() as usize + 1;
                if s % size != 0 { continue; }
                let num = repeat_number(i, s / size, size);
                if checked.contains(&num) { continue; }
                if num >= start && num <= end { result += num; }
                checked.insert(num);
            }
        }
    }

    Some(result)
}

const POWERS: [u64;11] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000];

// Takes a number 'e.g. 12' and repeats is 'times' times.
// For performance reasons the size of the number is given as a parameter, so it doesn't have to be recalculated (don't do this in actual code haha)
// e.g. repeat_number(12, 4, 2) = 12121212
// This algorithm is faster than converting to strings, concatenating and then parsing again
fn repeat_number(num: u64, times: usize, size: usize) -> u64 {
    let mut result = 0;
    let mut exp = 0;
    for _ in 0..times {
        result += num * POWERS[exp];
        exp += size;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
