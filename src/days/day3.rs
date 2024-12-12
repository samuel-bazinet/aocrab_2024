use crate::input_reader;

use regex::Regex;

type Input = String;

pub fn run_day() {
    println!("Day 3:");
    let input = process_input("inputs/day3.txt");
    println!("\tPart 1: {}", part_1(&input));
    println!("\tPart 2: {}", part_2(&input));
}

fn process_input(input: &str) -> Input {
    input_reader::read_lines(input)
        .into_iter()
        .fold(String::new(), |acc, e| acc + e.as_str())
}

fn part_1(input: &Input) -> u64 {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").expect("Invalid regex");
    re.captures_iter(input)
        .map(|caps| {
            caps.name("a").unwrap().as_str().parse::<u64>().unwrap()
                * caps.name("b").unwrap().as_str().parse::<u64>().unwrap()
        })
        .fold(0, |r, i| r + i)
}

fn part_2(input: &Input) -> u64 {
    // Use if let for each match type
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)|(do\(\))|(don't\(\))")
        .expect("Invalid regex");
    let mut result = 0;
    let mut enabled = true;
    re.find_iter(input).for_each(|mat| {
        match mat.len() {
            4 => enabled = true,
            7 => enabled = false,
            _ => {
                if enabled {
                    let caps = re.captures(mat.as_str()).unwrap();
                    result += caps.name("a").unwrap().as_str().parse::<u64>().unwrap()
                        * caps.name("b").unwrap().as_str().parse::<u64>().unwrap();
                }
            }
        };
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 161;
        let input = process_input("practice/day3_1.txt");
        let result = part_1(&input);
        assert_eq!(
            result, expected,
            "{result} is not the right answer, expected {expected}"
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 48;
        let input = process_input("practice/day3_2.txt");
        let result = part_2(&input);
        assert_eq!(
            expected, result,
            "{result} is not the right answer, expected {expected}"
        );
    }
}
