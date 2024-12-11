advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut part1result = 0;
    input.trim().split('\n').for_each(|line| {
        let mut direction: Option<Direction> = None;
        let mut values = line.split(' ').map(|s| s.parse::<usize>().unwrap());
        let mut prev = values.next().unwrap();
        let failed = values.fold(false, |fail, v| {
            if fail {
                return fail;
            };
            if v.abs_diff(prev) > 3 || v.abs_diff(prev) < 1 {
                return true;
            }

            let current_direction = match v > prev {
                true => Direction::Positive,
                false => Direction::Negative,
            };

            match direction {
                Some(d) => {
                    if d != current_direction {
                        return true;
                    }
                }
                None => direction = Some(current_direction),
            };
            prev = v;
            false
        });
        if !failed {
            part1result += 1
        };
    });
    Some(part1result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut part2result = 0;
    let rows: Vec<&str> = input.trim().split('\n').collect();
    'row: for row in rows {
        if row_is_safe(string_to_row(row)) {
            part2result += 1;
            continue 'row;
        }

        for permutation in row_to_partial_rows(string_to_row(row)) {
            if row_is_safe(permutation) {
                part2result += 1;
                continue 'row;
            }
        }
    }
    Some(part2result as u32)
}

fn abs_pass(a: usize, b: usize) -> bool {
    let diff = a.abs_diff(b);
    (1..=3).contains(&diff)
}

fn direction(a: usize, b: usize) -> Direction {
    if a < b {
        Direction::Positive
    } else {
        Direction::Negative
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Positive,
    Negative,
}

fn matches_direction(dir: Direction, a: usize, b: usize) -> bool {
    dir == direction(a, b)
}

fn row_is_safe(row: Vec<usize>) -> bool {
    if row.len() <= 2 {
        return true;
    }
    let dir = direction(row[0], row[1]);
    row.windows(2)
        .all(|arr| matches_direction(dir, arr[0], arr[1]) && abs_pass(arr[0], arr[1]))
}

fn string_to_row(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn row_to_partial_rows(row: Vec<usize>) -> Vec<Vec<usize>> {
    row.iter()
        .enumerate()
        .map(|(i, _)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, s)| if i == j { None } else { Some(*s) })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(585));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(626));
    }
}
