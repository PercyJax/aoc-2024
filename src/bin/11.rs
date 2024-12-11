use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let iterations = 25;

    let mut sum = 0;
    let mut table = HashMap::new();
    for value in input.trim().split(' ').map(|s| s.parse::<usize>().unwrap()) {
        sum += lookup_table(value, iterations, &mut table)
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let iterations = 75;

    let mut sum = 0;
    let mut table = HashMap::new();
    for value in input.trim().split(' ').map(|s| s.parse::<usize>().unwrap()) {
        sum += lookup_table(value, iterations, &mut table)
    }

    Some(sum)
}

fn blink_logic(number: usize) -> Vec<usize> {
    if number == 0 {
        vec![1]
    } else if number.to_string().len() % 2 == 0 {
        let vstr = number.to_string();
        let left = vstr.chars().collect::<Vec<char>>()[..(vstr.len() / 2)]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let right = vstr.chars().collect::<Vec<char>>()[(vstr.len() / 2)..]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        vec![left, right]
    } else {
        vec![number * 2024]
    }
}

// (number, iterations), total
fn lookup_table(
    number: usize,
    iterations: usize,
    table: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(result) = table.get(&(number, iterations)) {
        return *result;
    }
    let splits = blink_logic(number);
    let mut sum = 0;
    for split in splits {
        let Some(i) = iterations.checked_sub(1) else {
            return 1;
        };
        sum += lookup_table(split, i, table);
    }
    table.insert((number, iterations), sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
