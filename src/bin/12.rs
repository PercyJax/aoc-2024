use std::{collections::HashSet, fmt::Debug, hash::Hash};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let map = input_to_2d_vec(input);

    let mut region_list: Vec<(usize, usize)> = Vec::new(); // area, perimeter
    let mut region_map: Map<Option<usize>> = vec![vec![None; map[0].len()]; map.len()]; // coordinates, region_index

    map.iter().enumerate().for_each(|(r_idx, row)| {
        row.iter().enumerate().for_each(|(c_idx, _)| {
            // println!("({r_idx}), ({c_idx})");
            if map_lookup((r_idx, c_idx), None, &region_map).is_some_and(|(_, _, region)| {
                // println!("region: {region:?}");
                region.is_none()
            }) {
                let same_region = explore_region((r_idx, c_idx), &map);
                let area = same_region.len();
                let perimeter = same_region.iter().fold(0, |sum, (r, c)| {
                    region_map[*r][*c] = Some(region_list.len()); // length of region_list is the next region index
                    sum + usize::from(get_perimeter((*r, *c), &map))
                });
                // println!(
                //     "plot: {plot}, region: {}, area: {area}, perimeter: {perimeter}",
                //     region_list.len()
                // );
                region_list.push((area, perimeter));
            }
        })
    });

    let res = region_list
        .iter()
        .fold(0, |sum, (area, perimeter)| sum + (area * perimeter));

    Some(res)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn input_to_2d_vec(input: &str) -> Map<char> {
    input
        .trim()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect()
}

type Coordinates<T> = (T, T);
type Map<T> = Vec<Vec<T>>;

fn map_lookup<T, U>(
    coordinates: Coordinates<T>,
    offset: Option<(isize, isize)>,
    map: &Map<U>,
) -> Option<(usize, usize, U)>
where
    T: Copy,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    U: Copy,
{
    let Ok(row) = usize::try_from(coordinates.0) else {
        return None;
    };
    let Ok(col) = usize::try_from(coordinates.1) else {
        return None;
    };

    if let Some(offset) = offset {
        let n_row = row.checked_add_signed(offset.0)?;
        let n_col = col.checked_add_signed(offset.1)?;
        let n_plot = map.get(n_row).and_then(|r| r.get(n_col)).copied()?;
        return Some((n_row, n_col, n_plot));
    }

    let plot = map.get(row).and_then(|r| r.get(col)).copied()?;

    Some((row, col, plot))
}

/// Returns HashSet of coordinates that belong in the same region as the provided coordinates.
fn explore_region<T>(coordinates: Coordinates<T>, map: &Map<char>) -> HashSet<Coordinates<T>>
where
    T: Copy,
    T: Debug,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    (T, T): Eq,
    (T, T): Hash,
{
    let mut seen = HashSet::new();

    let mut neighbors = get_neighbors(coordinates, map);
    seen.insert(coordinates);
    // print!("starting with {coordinates:?} - ");

    while let Some(neighbor) = neighbors.pop() {
        if !seen.insert(neighbor) {
            continue;
        }
        let new_neighbors = get_neighbors(neighbor, map)
            .into_iter()
            .filter(|n| !seen.contains(n))
            .collect::<Vec<Coordinates<T>>>();
        neighbors.extend(new_neighbors);
    }

    // println!("found neighbors: {seen:?}");

    seen
}

fn get_perimeter<T>(coordinates: Coordinates<T>, map: &Map<char>) -> u8
where
    T: Copy,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    4_u8.checked_sub(get_neighbors(coordinates, map).len().try_into().unwrap())
        .unwrap()
}

fn get_neighbors<T>(coordinates: Coordinates<T>, map: &Map<char>) -> Vec<Coordinates<T>>
where
    T: Copy,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: Debug,
    T: TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
{
    let mut res = Vec::new();

    let (_, _, plot) = map_lookup(coordinates, None, map).unwrap();

    for offset in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let Some((n_row, n_col, n_plot)) = map_lookup(coordinates, Some(offset), map) else {
            continue;
        };

        if n_plot == plot {
            res.push((n_row.try_into().unwrap(), n_col.try_into().unwrap()));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
