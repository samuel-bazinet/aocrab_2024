use crate::input_reader;

type Input = Vec<Vec<i32>>;

pub fn run_day() {
    println!("Day 2:");
    let mut input = process_input("inputs/day2.txt");
    println!("\tPart 1: {}", part_1(&input));
    println!("\tPart 2: {}", part_2(&mut input));
}

fn process_input(input: &str) -> Input {
    input_reader::read_lines(input)
        .iter()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn part_1(input: &Input) -> i32 {
    input.iter().fold(0, |r, i| {
        if detect_valid(i) {r+1} else {r}
    })
}

fn part_2(input: &mut Input) -> i32 {
    input.iter_mut().fold(0, |r, list| {
        for i in 0..list.len() {
            let out = list.remove(i);
            if detect_valid(list) {
                return r+1;
            } 
            list.insert(i, out);
        }
        r
    })
}

fn detect_valid(i: &Vec<i32>) -> bool {
    let mut diffs = i.windows(2).map(|x| x[0] - x[1]);
    if diffs.clone().all(|x| 1 <= x.abs() && x.abs() <= 3) {
        if diffs.clone().all(|x| x > 0) || diffs.all(|x| x < 0) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 2;
        let input = process_input("practice/day2.txt");
        let result = part_1(&input);
        assert_eq!(
            expected, result,
            "{} is not the right answer, expected {}",
            result, expected
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 4;
        let mut input = process_input("practice/day2.txt");
        let result = part_2(&mut input);
        assert_eq!(
            expected, result,
            "{} is not the right answer, expected {}",
            result, expected
        );
    }
}
