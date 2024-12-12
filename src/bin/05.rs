use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, updates) = parse(input);
    let ordering_set = ordering_set_from_rules(&rules);

    let mut res = 0;
    'line: for update in updates {
        // if update
        //     .windows(2)
        //     .any(|u| violates_rules(u[0], u[1], &ordering_set))
        // {
        //     continue;
        // }
        let mut a_idx = 0;

        while a_idx < update.len() - 1 {
            let mut b_idx = a_idx + 1;
            while b_idx < update.len() {
                if violates_rules(update[a_idx], update[b_idx], &ordering_set) {
                    continue 'line;
                }
                b_idx += 1;
            }
            a_idx += 1;
        }
        res += update[(update.len() - 1) / 2]
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn violates_rules(a: usize, b: usize, ordering_set: &HashMap<(usize, usize), Ordering>) -> bool {
    ordering_set
        .get(&(a, b))
        .is_some_and(|o| *o == Ordering::Greater)
}

fn ordering_set_from_rules(rules: &Vec<(usize, usize)>) -> HashMap<(usize, usize), Ordering> {
    let mut comparison_set = HashMap::new();
    for rule in rules {
        let prev_f = comparison_set.insert(*rule, Ordering::Less);
        let prev_b = comparison_set.insert((rule.1, rule.0), Ordering::Greater);

        // just to make sure data makes intuitive sense
        assert!(prev_f.is_none() || prev_f.is_some_and(|v| v == Ordering::Less));
        assert!(prev_b.is_none() || prev_b.is_some_and(|v| v == Ordering::Greater));
    }
    comparison_set
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut parts = input.trim().split("\n\n");
    let rules = parts.next().unwrap();
    let updates = parts.next().unwrap();
    let res_rules = rules
        .trim()
        .split('\n')
        .map(|r| {
            let mut vals = r.split('|');
            (
                vals.next().unwrap().parse::<usize>().unwrap(),
                vals.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();
    let res_updates = updates
        .trim()
        .split('\n')
        .map(|r| {
            r.trim()
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (res_rules, res_updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
