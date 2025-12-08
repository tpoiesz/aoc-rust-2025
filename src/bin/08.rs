use advent_of_code::utils::disjoint_set::DisjointSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    const N_LONGEST_CIRCUITS: usize = 3;
    const N_SHORTEST_CONNECTIONS: usize = 1000;

    let boxes = parse_input(input);

    let distances = calculate_distances_sorted(&boxes);

    let mut circuits = DisjointSet::new(boxes.len());
    for (b1, b2, _) in distances.iter().take(N_SHORTEST_CONNECTIONS) {
        circuits.union(*b1, *b2);
    }

    let mut circuit_lengths = circuits.iter().map(|(_, size)| size as u64).collect::<Vec<_>>();
    circuit_lengths.sort_unstable();
    let result = circuit_lengths.iter().rev().take(N_LONGEST_CIRCUITS).product::<u64>();

    Some(result)
}

fn parse_input(input: &str) -> Vec<(u64, u64, u64)> {
    input.lines()
        .map(|l| {
            let as_vec = l
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (as_vec[0], as_vec[1], as_vec[2])
        })
        .collect::<Vec<_>>()
}

// For this problem we don't care about the actual distance, only it's ordering, so we can skip taking the square root for performance
fn euclid_distance_squared(b1: &(u64, u64, u64), b2: &(u64, u64, u64)) -> u64 {
    let d0 = b1.0.abs_diff(b2.0);
    let d1 = b1.1.abs_diff(b2.1);
    let d2 = b1.2.abs_diff(b2.2);
    d0 * d0 + d1 * d1 + d2 * d2
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_input(input);

    let distances = calculate_distances_sorted(&boxes);

    let mut circuits = DisjointSet::new(boxes.len());
    let mut result = 0;

    for (b1, b2, _) in distances {
        if circuits.union(b1, b2) == boxes.len() {
            result = boxes[b1].0 * boxes[b2].0;
            break;
        };
    }

    Some(result)
}

fn calculate_distances_sorted(boxes: &Vec<(u64, u64, u64)>) -> Vec<(usize, usize, u64)> {
    let mut distances = Vec::with_capacity(boxes.len() * (boxes.len() - 1));

    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            distances.push((i, j, euclid_distance_squared(&boxes[i], &boxes[j])));
        }
    }

    distances.sort_unstable_by_key(|(_, _, distance)| *distance);
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
