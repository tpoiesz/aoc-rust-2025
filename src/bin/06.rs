advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut problems = Vec::new();

    let lines = input.lines().collect::<Vec<&str>>();
    for number_line in &lines[..lines.len() - 1] {
        for (idx, num) in number_line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .enumerate() {
            match problems.get_mut(idx) {
                None => problems.push(vec![num]),
                Some(nums) => nums.push(num),
            }
        }
    }

    let mut result = 0;

    for (idx, operator) in lines.last().unwrap().split_ascii_whitespace().enumerate() {
        match operator {
            "+" => result += problems[idx].iter().sum::<u64>(),
            "*" => result += problems[idx].iter().product::<u64>(),
            _ => panic!(),
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut problems = Vec::new();

    let lines = input.lines().collect::<Vec<&str>>();
    let numbers = lines[..lines.len() - 1].iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let num_cols = numbers.iter().max_by_key(|l| l.len()).unwrap().len();
    let num_rows = numbers.len();
    let mut number_index = 0;
    problems.push(Vec::new());
    for column in 0..num_cols {
        let mut num = None;
        for row in 0..num_rows {
            let maybe_char = numbers[row].get(column);
            if maybe_char.is_some_and(|c| c.is_ascii_digit()) {
                let digit = (maybe_char.unwrap() - b'0') as u64;
                num = num.map(|n| 10*n + digit).or(Some(digit));
            }
        }
        match num {
            None => {
                number_index += 1;
                problems.push(Vec::new());
            }
            Some(n) => { problems[number_index].push(n) }
        }
    }

    // println!("{:?}", problems);

    let mut result = 0;

    for (idx, operator) in lines.last().unwrap().split_ascii_whitespace().enumerate() {
        match operator {
            "+" => result += problems[idx].iter().sum::<u64>(),
            "*" => result += problems[idx].iter().product::<u64>(),
            _ => panic!(),
        }
    }

    Some(result)
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
