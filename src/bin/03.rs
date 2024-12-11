advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    input.trim().split('\n').for_each(|row| {
        // state machine
        let mut p = ParserMachine {
            buffer: String::new(),
            state: State::Blank,
            should_do: true,
        };
        result += row.chars().filter_map(|c| p.ingest(c)).sum::<u32>();
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    // state machine
    let mut p = ParserMachine {
        buffer: String::new(),
        state: State::Blank,
        should_do: true,
    };
    input.trim().split('\n').for_each(|row| {
        result += row
            .chars()
            .filter_map(|c| p.conditional_ingest(c))
            .sum::<u32>();
    });

    Some(result)
}

struct ParserMachine {
    buffer: String,
    state: State,
    should_do: bool,
}

enum State {
    Blank,
    Should,
    Mul,
}

impl ParserMachine {
    fn conditional_ingest(&mut self, character: char) -> Option<u32> {
        let result = self.ingest(character);
        if self.should_do {
            result
        } else {
            None
        }
    }
    // return a total when a valid mul() command is completely ingested
    fn ingest(&mut self, character: char) -> Option<u32> {
        match self.state {
            State::Blank => match character {
                'm' => {
                    self.state = State::Mul;
                    self.buffer.push(character);
                    None
                }
                'd' => {
                    self.state = State::Should;
                    self.buffer.push(character);
                    None
                }
                _ => self.start_over(),
            },
            State::Should => {
                // discard anything that is not in [don't()]
                if !"don't()"
                    .chars()
                    .collect::<Vec<char>>()
                    .contains(&character)
                {
                    return self.start_over();
                }

                // discard anything that is out of place
                let valid = match self.buffer.len() {
                    0 => character == 'd',
                    1 => character == 'o',
                    2 => character == '(' || character == 'n',
                    3 => {
                        if self.buffer == "do(" && character == ')' {
                            self.should_do = true;
                            return self.start_over();
                        } else {
                            self.buffer == "don" && character == '\''
                        }
                    }
                    4..=5 => "don't(".contains(&format!("{}{}", self.buffer, character)),
                    6 => {
                        if character == ')' {
                            self.should_do = false;
                            return self.start_over();
                        } else {
                            false
                        }
                    }
                    7.. => false,
                };

                if !valid {
                    return self.start_over();
                } else {
                    self.buffer.push(character);
                }
                None
            }
            State::Mul => {
                // discard anything that is not in [mul()0123456789,]
                if !"mul()0123456789,"
                    .chars()
                    .collect::<Vec<char>>()
                    .contains(&character)
                {
                    return self.start_over();
                }

                // discard anything that is out of place
                let valid = match self.buffer.len() {
                    0 => character == 'm',
                    1 => character == 'u',
                    2 => character == 'l',
                    3 => character == '(',
                    4..=12 => {
                        let vals = &self.buffer[4..];
                        let mut valsplit = vals.split(',');
                        let left = valsplit.next().and_then(|v| v.parse::<u32>().ok());
                        let right = valsplit.next().and_then(|v| v.parse::<u32>().ok());
                        match character {
                            x if x.is_numeric() => {
                                left.is_none()
                                    || (right.is_none()
                                        && left.is_some_and(|v| (0..=99).contains(&v)))
                                    || (right.is_none() && vals.contains(','))
                                    || right.is_some_and(|v| (0..=99).contains(&v))
                            }
                            ',' => !vals.contains(',') && left.is_some(),
                            ')' => {
                                if let (Some(left), Some(right)) = (left, right) {
                                    self.start_over();
                                    return Some(left * right);
                                }
                                return self.start_over();
                            }
                            _ => return self.start_over(),
                        }
                    }
                    13.. => false,
                };

                if !valid {
                    return self.start_over();
                } else {
                    self.buffer.push(character);
                }
                None
            }
        }
    }
    fn start_over(&mut self) -> Option<u32> {
        self.buffer = String::new();
        self.state = State::Blank;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
