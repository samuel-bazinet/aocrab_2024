use std::collections::HashMap;

use crate::input_reader;

type Input = HashMap<(isize, isize), u8>;

const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

const CHARS: [u8; 3] = [b'M', b'A', b'S'];

const TARGET: u8 = b'M' + b'S';

pub fn run_day() {
    println!("Day :");
    let input = process_input("inputs/day4.txt");
    println!("\tPart 1: {}", part_1(&input));
    println!("\tPart 2: {}", part_2(&input));
}

fn process_input(input: &str) -> Input {
    let mut out = HashMap::new();
    input_reader::read_lines(input)
        .iter()
        .enumerate()
        .for_each(|(y, row)| {
            row.clone()
                .into_bytes()
                .iter()
                .enumerate()
                .for_each(|(x, char)| {
                    out.insert((y as isize, x as isize), *char);
                });
        });
    out
}

fn part_1(input: &Input) -> u64 {
    input.iter().fold(0, |r, r#in| {
        if r#in.1 == &(b'X') {
            r + find_xmas(input, r#in.0)
        } else {
            r
        }
    })
}

fn find_xmas(input: &Input, start: &(isize, isize)) -> u64 {
    let mut out = 0;
    for dir in DIRS {
        let mut valid = true;
        for (i, char) in CHARS.iter().enumerate() {
            let coord = &(
                start.0 + dir.0 * (i as isize + 1),
                start.1 + dir.1 * (i as isize + 1),
            );
            if input.contains_key(coord) {
                if input[coord] != *char {
                    valid = false;
                }
            } else {
                valid = false;
            }
        }
        if valid {
            out += 1;
        }
    }
    out
}

fn part_2(input: &Input) -> u64 {
    input.iter().fold(0, |r, r#in| {
        if r#in.1 == &(b'A') {
            if find_x_mas(input, r#in.0) {
                r + 1
            } else {
                r
            }
        } else {
            r
        }
    })
}

fn find_x_mas(input: &Input, start: &(isize, isize)) -> bool {
    let coord_a = &(start.0 + -1, start.1 + -1);
    let coord_b = &(start.0 + 1, start.1 + 1);
    let coord_c = &(start.0 + -1, start.1 + 1);
    let coord_d = &(start.0 + 1, start.1 + -1);

    (input.contains_key(coord_a) && input.contains_key(coord_b))
        && (input[coord_a] + input[coord_b] == TARGET)
        && (input.contains_key(coord_c) && input.contains_key(coord_d))
        && (input[coord_c] + input[coord_d] == TARGET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 18;
        let input = process_input("practice/day4.txt");
        let result = part_1(&input);
        assert_eq!(
            result, expected,
            "{result} is not the right answer, expected {expected}"
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 9;
        let input = process_input("practice/day4.txt");
        let result = part_2(&input);
        assert_eq!(
            result, expected,
            "{result} is not the right answer, expected {expected}"
        );
    }
}
