use std::collections::{HashMap, HashSet, VecDeque};
use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::position::{Direction, Position};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse(input);
    // let mut debug = grid.clone();
    // for item in debug.iter_mut() {
    //     if item == &b'^' { *item = b'0' }
    // }
    let mut result = 0;

    let mut beam_starts = VecDeque::new();
    let mut visited = HashSet::new();
    let start = grid.position(&b'S').unwrap();
    beam_starts.push_back(start);

    while let Some(mut position) = beam_starts.pop_front() {
        loop {
            if !visited.insert(position) { break; }
            // debug.set(position, b'|');
            position = position.step(Direction::Down);
            let next = grid.get(position);
            if next.is_none() { break; }
            let next = next.unwrap();
            if next == &b'^' {
                // println!("hit: {:?}", position);
                result += 1;
                let left = position.step(Direction::Left);
                let right = position.step(Direction::Right);
                beam_starts.push_back(left);
                beam_starts.push_back(right);
                // debug.set(position, debug.get(position).unwrap() + 1);
                break;
            }
        }
    }

    // println!("{debug:#}");

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::parse(input);
    let first_splitter = grid.position(&b'^').unwrap();
    let mut timelines_from = HashMap::new();

    let last_splitter_row_index = grid.height - 2;
    for column in 0..grid.width {
        let position = Position::new(last_splitter_row_index as i32, column as i32);
        if grid.get(position).unwrap() == &b'^' {
            timelines_from.insert(position, 2u64);
        }
    }

    // println!("{:?}", timelines_from);

    for row in (0..last_splitter_row_index - 1).step_by(2).rev() {
        for column in 0..grid.width {
            let position = Position::new(row as i32, column as i32);
            if grid.get(position).unwrap() == &b'^' {
                // println!("{position:?}");
                let mut left_timelines = 1;
                let mut pos = position.step(Direction::Left).step(Direction::Down);
                while let Some(c) = grid.get(pos) {
                    if c == &b'^' {
                        left_timelines = timelines_from[&pos];
                        break;
                    }
                    pos = pos.step(Direction::Down);
                }

                let mut right_timelines = 1;
                let mut pos = position.step(Direction::Right).step(Direction::Down);
                while let Some(c) = grid.get(pos) {
                    if c == &b'^' {
                        right_timelines = timelines_from[&pos];
                        break;
                    }
                    pos = pos.step(Direction::Down);
                }

                timelines_from.insert(position, left_timelines + right_timelines);
            }
        }
    }

    Some(*timelines_from.get(&first_splitter).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
