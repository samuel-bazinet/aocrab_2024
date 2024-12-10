use crate::input_reader;

type Input = Vec<String>;

pub fn run_day() {
    println!("Day :");
    let input = process_input("inputs/day.txt");
    println!("\tPart 1: {}", part_1(&input));
    println!("\tPart 2: {}", part_2(&input));
}

fn process_input(input: &str) -> Input {
    vec![]
}

fn part_1(input: &Input) -> u64 {
    0
}

fn part_2(input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 0;
        let input = process_input("practice/day.txt");
        let result = part_1(&input);
        assert_eq!(
            expected, result,
            "{} is not the right answer, expected {}",
            result, expected
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 0;
        let input = process_input("practice/day.txt");
        let result = part_2(&input);
        assert_eq!(
            expected, result,
            "{} is not the right answer, expected {}",
            result, expected
        );
    }
}