advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let mut position = 50;

    // Only last two characters approach

    // for instruction in input.lines() {
    //     let num_string = &instruction[1..];
    //     let len = num_string.len();
    //     let num : i32 =
    //         if len <= 2 { num_string.parse().unwrap() }
    //         else { num_string[len - 2..].parse().unwrap() }; //only parse last two characters
    //     if instruction.starts_with('R') { position += num; }
    //     else { position -= num; }
    //     if position >= 100 { position -= 100; }
    //     if position < 0 { position += 100; }
    //     if position == 0 { count += 1; }
    // }

    // Modulo approach

    // for instruction in input.lines() {
    //     let num = instruction[1..].parse::<i32>().unwrap();
    //     if instruction.starts_with('R') { position += num; }
    //     else { position -= num; }
    //     position = position % 100;
    //     if position == 0 { count += 1; }
    // }

    // Keep counting method (also with modulo),
    // Turns out it is unnecessary to keep track of the 'real' dial number (e.g. -88, 12 and 212 all correspond to dial number 12)
    // Saves a single line, probably does not have any performance impact though

    for instruction in input.lines() {
        let num = instruction[1..].parse::<i32>().unwrap();
        if instruction.starts_with('R') { position += num; }
        else { position -= num; }
        if  position % 100 == 0 { count += 1; }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count : u64 = 0;
    let mut position = 50;

    for instruction in input.lines() {
        let num = instruction[1..].parse::<i32>().unwrap();
        // println!("{}, {}", position, instruction);
        if instruction.starts_with('R') {
            let remaining_after_zero = num - (100 - position);
            if remaining_after_zero >= 0 { count += (remaining_after_zero / 100) as u64 + 1 }
            position = (position + num) % 100;
        }
        else {
            let remaining_after_zero = if position == 0 { num - 100 } else { num - position };
            if remaining_after_zero >= 0 { count += (remaining_after_zero / 100) as u64 + 1 }
            position = position - (num % 100);
            if position < 0 { position = position + 100; }
        };
        // println!("{}, {}", position, count);
    }

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
