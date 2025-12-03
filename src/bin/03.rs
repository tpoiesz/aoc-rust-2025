advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result : u64 = 0;
    for line in input.lines() {
        result += biggest(line, 2);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result : u64 = 0;
    for line in input.lines() {
        result += biggest(line, 12);
    }

    Some(result)
}

fn biggest(line: &str, size: usize) -> u64 {
    let mut digits = line.as_bytes();
    let mut result : u64 = 0;

    // The standard 'max()' function returns the last element if multiple elements have the same max value
    // This returns the first element if multiple elements have the same max value
    let find_first_max = |digits: &[u8]| digits.iter().enumerate()
        .fold((0, 0), |(max_idx, max), (idx, &num)|
            if num > max { (idx, num) }
            else { (max_idx, max) });

    for position in 0..size {
        let right_limit = size - position - 1;
        let (max_idx, max) = find_first_max(&digits[0..digits.len() - right_limit]);
        result = result * 10 + (max - b'0') as u64;
        digits = &digits[max_idx + 1..];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
