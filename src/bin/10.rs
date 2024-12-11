use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = input_to_2d_vec(input.to_owned());
    let seeds = locate(&map, 0);
    Some(find_global_unique_summits(seeds, &map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input_to_2d_vec(input.to_owned());
    let seeds = locate(&map, 0);
    Some(find_all_paths_to_summit(seeds, &map))
}

fn input_to_2d_vec(input: String) -> Vec<Vec<u8>> {
    input
        .trim()
        .split('\n')
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn compute_valid_moves(
    coordinates: (usize, usize),
    map: &[Vec<u8>],
    mut summits: Option<&mut HashSet<(usize, usize)>>,
    mut valid_paths: Option<&mut usize>,
) -> Vec<(usize, usize)> {
    let current_number = *map.get(coordinates.0).unwrap().get(coordinates.1).unwrap();

    let mut moves = Vec::new();

    // up, down, left, right
    for (r, c) in [
        (((coordinates.0).checked_sub(1)), Some(coordinates.1)),
        (((coordinates.0).checked_add(1)), Some(coordinates.1)),
        (Some(coordinates.0), (coordinates.1).checked_sub(1)),
        (Some(coordinates.0), (coordinates.1).checked_add(1)),
    ] {
        let (Some(r), Some(c)) = (r, c) else {
            continue;
        };
        if let Some(row) = map.get(r) {
            if let Some(value) = row.get(c) {
                if *value != current_number + 1 {
                    continue;
                }
                if *value == 9 {
                    if let Some(ref mut summits) = summits {
                        (*summits).insert((r, c));
                    }
                    if let Some(ref mut valid_paths) = valid_paths {
                        **valid_paths += 1;
                    }
                } else {
                    moves.push((r, c));
                }
            }
        }
    }

    moves
}

fn locate(map: &[Vec<u8>], number: u8) -> Vec<(usize, usize)> {
    let mut coordinates = Vec::new();
    map.iter().enumerate().for_each(|(row, values)| {
        values.iter().enumerate().for_each(|(column, value)| {
            if *value == number {
                let (r, c) = (row, column);
                coordinates.push((r, c));
            }
        })
    });
    coordinates
}

fn search_for_unique_summits(
    seed: (usize, usize),
    map: &[Vec<u8>],
    summits: &mut HashSet<(usize, usize)>,
) {
    let mut moves = Vec::new();
    moves.push(seed);
    loop {
        let Some(next) = moves.pop() else { break };
        moves.extend(compute_valid_moves(next, map, Some(summits), None));
    }
}

fn search_for_unique_paths(seed: (usize, usize), map: &[Vec<u8>], paths: &mut usize) {
    let mut moves = Vec::new();
    moves.push(seed);
    loop {
        let Some(next) = moves.pop() else {
            break;
        };
        moves.extend(compute_valid_moves(next, map, None, Some(paths)));
    }
}

fn find_global_unique_summits(seeds: Vec<(usize, usize)>, map: &[Vec<u8>]) -> u32 {
    let mut result = 0;
    for seed in seeds {
        let mut summits = HashSet::new();
        search_for_unique_summits(seed, map, &mut summits);
        result += summits.len();
    }
    result as u32
}

fn find_all_paths_to_summit(seeds: Vec<(usize, usize)>, map: &[Vec<u8>]) -> u32 {
    let mut result = 0;
    for seed in seeds {
        let mut paths = 0;
        search_for_unique_paths(seed, map, &mut paths);
        result += paths;
    }
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(825));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1805));
    }
}
