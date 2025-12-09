use std::cmp::{max, min};
use std::collections::HashSet;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let pairs = parse_points(input);

    let mut max = 0;
    for i in 0..pairs.len()-1 {
        for j in i+1..pairs.len() {
            let size = calculate_area(pairs[i], pairs[j]);
            if size > max { max = size; }
        }
    }

    Some(max)
}

struct HorizontalLine {
    row: usize,
    col_start: usize,
    col_end: usize
}

struct VerticalLine {
    column: usize,
    row_start: usize,
    row_end: usize
}

pub fn part_two(input: &str) -> Option<u64> {
    let pairs = parse_points(input);
    let mut pair_set = HashSet::new();

    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();
    let mut p1 = *pairs.last().unwrap();
    let mut p2 = *pairs.first().unwrap();
    for i in 0..pairs.len()-1 {
        pair_set.insert(p1);
        pair_set.insert(p2);
        if p1.1 == p2.1 {
            horizontal_lines.push(HorizontalLine { row: p1.1, col_start: min(p1.0, p2.0), col_end: max(p1.0, p2.0) });
        } else {
            vertical_lines.push(VerticalLine { column: p1.0, row_start: min(p1.1, p2.1), row_end: max(p1.1, p2.1) });
        }
        (p1, p2) = (pairs[i], pairs[i+1]);
    }

    let mut sizes = Vec::with_capacity(pairs.len() * (pairs.len() - 1));
    for i in 0..pairs.len()-1 {
        for j in i+1..pairs.len() {
            let size = calculate_area(pairs[i], pairs[j]);
            sizes.push((i, j, size));
        }
    }
    sizes.sort_unstable_by_key(|(_, _, area)| *area);

    for (i, j, size) in sizes.iter().rev() {
        let (p1, p2) = (pairs[*i], pairs[*j]);
        let c1 = (p1.0, p2.1);
        let c2 = (p2.0, p1.1);

        if !pair_set.contains(&c1) &&
            vertical_lines.iter().find(|l| l.column == c1.0 && l.row_start < c1.1 && l.row_end > c1.1).is_none() &&
            horizontal_lines.iter().find(|l| l.row == c1.1 && l.col_start < c1.0 && l.col_end > c1.0).is_none() {
            let vs = vertical_lines.iter()
                .filter(|l| l.column < c1.0 && l.row_start <= c1.1 && l.row_end >= c1.1)
                .collect::<Vec<_>>();
            let mut count = vs.len();
            if count == 0 { continue; }
            for i in 0..vs.len()-1 {
                for j in i+1..vs.len() {
                    if vs[i].row_start == c1.1 && vs[j].row_end == c1.1 ||
                        vs[i].row_end == c1.1 && vs[j].row_start == c1.1 { count -= 1; }
                }
            }
            if count % 2 == 0 { continue; }
        }
        if !pair_set.contains(&c2) &&
            vertical_lines.iter().find(|l| l.column == c2.0 && l.row_start < c2.1 && l.row_end > c2.1).is_none() &&
            horizontal_lines.iter().find(|l| l.row == c2.1 && l.col_start < c2.0 && l.col_end > c2.0).is_none() {
            let vs = vertical_lines.iter()
                .filter(|l| l.column < c2.0 && l.row_start <= c2.1 && l.row_end >= c2.1)
                .collect::<Vec<_>>();
            let mut count = vs.len();
            if count == 0 { continue; }
            for i in 0..vs.len()-1 {
                for j in i+1..vs.len() {
                    if vs[i].row_start == c2.1 && vs[j].row_end == c2.1 ||
                        vs[i].row_end == c2.1 && vs[j].row_start == c2.1 { count -= 1; }
                }
            }
            if count % 2 == 0 { continue; }
        }

        let bottom_row = max(p1.1, p2.1);
        let top_row = min(p1.1, p2.1);
        let left_col = min(p1.0, p2.0);
        let right_col = max(p1.1, p2.1);

        if vertical_lines.iter().any(|line| {
            line.row_start < bottom_row && line.row_end >= bottom_row && line.column > left_col && line.column < right_col
        }) { continue; }

        if vertical_lines.iter().any(|line| {
            line.row_start <= top_row && line.row_end > top_row && line.column > left_col && line.column < right_col
        }) { continue; }

        if horizontal_lines.iter().any(|line| {
            line.col_start <= left_col && line.col_end > left_col && line.row > top_row && line.row < bottom_row
        }) { continue; }

        if horizontal_lines.iter().any(|line| {
            line.col_start < right_col && line.col_start >= right_col && line.row > top_row && line.row < bottom_row
        }) { continue; }
        
        return Some(*size)
    }

    None
}

fn calculate_area(p1: (usize, usize), p2: (usize, usize)) -> u64 {
    ((p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)) as u64
}

fn parse_points(input: &str) -> Vec<(usize, usize)> {
    let pairs = input.lines()
        .map(|l| {
            let split = l.split_once(',').unwrap();
            (split.0.parse::<usize>().unwrap(), split.1.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
