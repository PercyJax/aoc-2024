advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let map = input_to_2d_vec(input);
    let x_coordinates = locate(&map, 'X');
    Some(x_coordinates.iter().fold(0, |counter, coordinates| {
        counter + count_all_valid_xmas(&map, *coordinates)
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input_to_2d_vec(input);
    let a_coordinates = locate(&map, 'A');
    Some(a_coordinates.iter().fold(0, |counter, coordinates| {
        counter
            + if count_all_valid_mas(&map, *coordinates) {
                1
            } else {
                0
            }
    }))
}

type Coordinates = (isize, isize);

fn input_to_2d_vec(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split('\n')
        .map(|row| row.chars().collect())
        .collect()
}

fn locate(map: &[Vec<char>], character: char) -> Vec<Coordinates> {
    let mut res = Vec::new();
    map.iter().enumerate().for_each(|(row, values)| {
        values.iter().enumerate().for_each(|(column, value)| {
            if *value == character {
                let (r, c) = (
                    isize::try_from(row).unwrap(),
                    isize::try_from(column).unwrap(),
                );
                res.push((r, c));
            }
        })
    });
    // println!("locate: {res:?}");
    res
}

fn count_all_valid_xmas(map: &[Vec<char>], coordinates: Coordinates) -> usize {
    let mut res = 0;
    for direction in [
        Direction::Right,
        Direction::DownRight,
        Direction::Down,
        Direction::DownLeft,
        Direction::Left,
        Direction::UpLeft,
        Direction::Up,
        Direction::UpRight,
    ] {
        let next_coordinates = get_next_xmas_coordinates(coordinates, direction);
        // filter out invalid coordinates
        if !next_coordinates.iter().all(|c| {
            usize::try_from(c.0).is_ok_and(|c| c < map.len())
                && usize::try_from(c.1).is_ok_and(|c| c < map[0].len()) // assumes map has at least one line
        }) {
            continue;
        }
        assert_eq!(next_coordinates.len(), 3);
        if next_coordinates
            .iter()
            .zip("MAS".chars())
            .all(|((row, col), expected_character)| {
                // println!("row: {row}, col: {col}");
                map[usize::try_from(*row).unwrap()][usize::try_from(*col).unwrap()]
                    == expected_character
            })
        {
            res += 1
        };
    }
    // println!("count_all_valid_xmas: {res:?}");
    res
}

fn count_all_valid_mas(map: &[Vec<char>], coordinates: Coordinates) -> bool {
    (is_valid_mas(coordinates, map, Direction::DownRight)
        || is_valid_mas(coordinates, map, Direction::UpLeft))
        && (is_valid_mas(coordinates, map, Direction::DownLeft)
            || is_valid_mas(coordinates, map, Direction::UpRight))
}

fn get_next_xmas_coordinates(root: Coordinates, direction: Direction) -> Vec<Coordinates> {
    let mut res = Vec::new();
    // skip 0
    for distance in 1..=3 {
        res.push((
            root.0 + (direction.unit().0 * distance),
            root.1 + (direction.unit().1 * distance),
        ));
    }
    // println!("get_next_coordinates: {res:?}");
    res
}

fn get_next_mas_coordinates(root: Coordinates, direction: Direction) -> [Coordinates; 2] {
    let mut res: [Coordinates; 2] = [(0, 0), (0, 0)];
    for (index, distance) in [-1, 1].iter().enumerate() {
        res[index] = (
            root.0 + (direction.unit().0 * distance),
            root.1 + (direction.unit().1 * distance),
        );
    }
    // println!("get_next_coordinates: {res:?}");
    res
}

fn is_valid_mas(coordinates: Coordinates, map: &[Vec<char>], direction: Direction) -> bool {
    let coordinates = get_next_mas_coordinates(coordinates, direction);
    coordinates.iter().all(|(row, col)| {
        usize::try_from(*row).is_ok_and(|row| row < map.len())
            && usize::try_from(*col).is_ok_and(|col| col < map[0].len())
    }) && map[usize::try_from(coordinates[0].0).unwrap()]
        [usize::try_from(coordinates[0].1).unwrap()]
        == 'M'
        && map[usize::try_from(coordinates[1].0).unwrap()]
            [usize::try_from(coordinates[1].1).unwrap()]
            == 'S'
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Up,
    UpRight,
}

impl Direction {
    fn unit(&self) -> Coordinates {
        match self {
            Direction::Right => (0, 1),
            Direction::DownRight => (1, 1),
            Direction::Down => (1, 0),
            Direction::DownLeft => (1, -1),
            Direction::Left => (0, -1),
            Direction::UpLeft => (-1, -1),
            Direction::Up => (-1, 0),
            Direction::UpRight => (-1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
