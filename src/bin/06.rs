advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let operators = lines.pop().unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();
    let numbers = lines
        .iter()
        .map(|line| line
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<u64>())
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let mut result = 0;

    for (idx, &operator) in operators.iter().enumerate() {
        let numbers = numbers.iter().map(|nums| nums[idx]);
        result += match operator {
            "+" => numbers.sum::<u64>(),
            "*" => numbers.product(),
            _ => unreachable!()
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let operator_line = lines.pop().unwrap();
    let mut result = 0;

    let num_cols = lines.iter().map(|line| line.len()).max().unwrap();
    let mut operator = b'.';

    let mut problem_outcome = None;
    for column in 0..num_cols {
        if problem_outcome.is_none() {
            operator = operator_line[column];
        }

        let number_in_column = parse_column(&lines, column);

        // If num is None the column was empty so we reached the end of the current problem
        match number_in_column {
            None => {
                result += problem_outcome.unwrap();
                problem_outcome = None;
            },
            Some(n) => problem_outcome = problem_outcome.map(|current| match operator {
                b'+' => current + n,
                b'*' => current * n,
                _ => panic!(),
            }).or(Some(n)),
        }
    }
    // For latest problem we never reach an empty column so adjust result with final problem outcome
    result += problem_outcome.unwrap();

    Some(result)
}

fn parse_column(lines: &Vec<&[u8]>, column: usize) -> Option<u64> {
    let mut column_number = None;
    for row in 0..lines.len() {
        let maybe_char = lines[row].get(column); //use Option because not all rows are equally long
        if maybe_char.is_some_and(|c| c.is_ascii_digit()) {
            let digit = (maybe_char.unwrap() - b'0') as u64;
            column_number = column_number.map(|n| 10 * n + digit).or(Some(digit));
        }
    }
    column_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
