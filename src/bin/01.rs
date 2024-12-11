use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();
    input.trim().split('\n').for_each(|line| {
        let mut tup = line.split("   ");
        list1.push(tup.next().unwrap().trim().parse().unwrap());
        list2.push(tup.next().unwrap().trim().parse().unwrap());
    });
    list1.sort();
    list2.sort();
    let result: usize = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();
    input.trim().split('\n').for_each(|line| {
        let mut tup = line.split("   ");
        list1.push(tup.next().unwrap().trim().parse().unwrap());
        list2.push(tup.next().unwrap().trim().parse().unwrap());
    });
    list1.sort();
    list2.sort();
    let mut occurrences: HashMap<usize, usize> = HashMap::new();
    list2.into_iter().for_each(|v| {
        occurrences.insert(v, *occurrences.get(&v).unwrap_or(&0) + 1);
    });
    let result: usize = list1
        .into_iter()
        .map(|v| v * occurrences.get(&v).unwrap_or(&0))
        .sum();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3714264));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18805872));
    }
}
