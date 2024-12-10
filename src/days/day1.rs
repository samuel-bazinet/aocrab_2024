use crate::input_reader;

type Input = (Vec<i32>, Vec<i32>);

pub fn run_day() {
    println!("Day 1:");
    let input = process_input("inputs/day1.txt");
    println!("\tPart 1: {}", part_1(&input));
    println!("\tPart 2: {}", part_2(&input));
}

fn process_input(input: &str) -> Input {
    let lines = input_reader::read_lines(input);
    let mut left = Vec::new();
    let mut right = Vec::new();

    lines.iter().for_each(|line| {
        let line = line.split("   ").collect::<Vec<_>>();
        left.push(line[0].parse().unwrap());
        right.push(line[1].parse().unwrap());
    });

    (left, right)
}

fn part_1(input: &Input) -> i32 {
    let mut left = input.0.clone();
    left.sort();
    let mut right = input.1.clone();
    right.sort();
    left.iter()
        .zip(right.iter())
        .fold(0, |r, i| r + (i.0 - i.1).abs())
}

fn part_2(input: &Input) -> i32 {
    let left = input.0.clone();
    let right = input.1.clone();
    left.iter().map(|x| {
        (x, right.iter().filter(move |y| x == *y).collect::<Vec<_>>().len())
    }).fold(0, |r, i| r + (i.0*i.1 as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 11;
        let input = process_input("practice/day1.txt");
        let result = part_1(&input);
        assert_eq!(
            expected, result,
            "{} is not the right answer, expected {}",
            result, expected
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 31;
        let input = process_input("practice/day1.txt");
        let result = part_2(&input);
        assert_eq!(
            expected, result,
            "{} is not the right answer, expected {}",
            result, expected
        );
    }
}
