use advent_of_code::utils::grid::Grid;
use advent_of_code::utils::position::ALL_DIRECTIONS;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::parse(input);
    let mut result = 0;

    for (pos, &elem) in grid.enumerate() {
        if elem != b'@' { continue; }
        if ALL_DIRECTIONS.iter().filter(|&direction| {
            grid.get(pos.step(*direction)).is_some_and(|&elem| elem == b'@')
        }).count() < 4 { result += 1; }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::parse(input);
    let mut result = 0;

    loop {
        let mut to_remove = Vec::new();
        for (pos, &elem) in grid.enumerate() {
            if elem != b'@' { continue; }
            if ALL_DIRECTIONS.iter()
                .filter(|&direction| {
                    grid.get(pos.step(*direction)).is_some_and(|&elem| elem == b'@')
                })
                .count() < 4 {
                    to_remove.push(pos);
                    result += 1;
                }
        }
        if to_remove.is_empty() { break; }
        for pos in to_remove { grid.set(pos, b'.')}
    }


    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
