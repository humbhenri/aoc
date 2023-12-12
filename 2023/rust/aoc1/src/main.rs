use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!(
        "{}",
        part1("/home/humberto/projects/aoc/2023/01.input").unwrap()
    );
}

fn part1(file_name: &str) -> io::Result<u32> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| {
            let (first_digit, last_digit) = get_first_and_last_digit(line.unwrap());
            first_digit * 10 + last_digit
        })
        .sum::<u32>())
}

fn get_first_and_last_digit(line: String) -> (u32, u32) {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    (first, digits.last().unwrap_or(first))
}

fn get_first_and_last_digit_part2(line: String) -> (u32, u32) {
    let patterns = "one two three four five six seven eight nine 1 2 3 4 5 6 7 8 9".split(' ');
    let mut first_digit = 0;
    let mut last_digit = 0;
    let mut first_digit_found = false;
    let mut last_digit_found = false;
    for p in patterns {
        if !first_digit_found && line.find(p).is_some() {
            first_digit = translate(p).unwrap();
            first_digit_found = true;
        }
        if !last_digit_found && line.rfind(p).is_some() {
            last_digit = translate(p).unwrap();
            last_digit_found = true;
        }
    }
    (first_digit, last_digit)
}

fn translate(p: &str) -> Option<u32> {
    match p {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        d => d.chars().find_map(|c| c.to_digit(10)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_and_last_digit_part2_test() {
        assert_eq!(
            (2, 9),
            get_first_and_last_digit_part2("two1nine".to_owned())
        );
    }

    #[test]
    fn get_first_and_last_digit_test() {
        assert_eq!((1, 2), get_first_and_last_digit("1abc2".to_owned()));
        assert_eq!((3, 8), get_first_and_last_digit("pqr3stu8vwx".to_owned()));
        assert_eq!((1, 5), get_first_and_last_digit("a1b2c3d4e5f".to_owned()));
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            55208,
            part1("/home/humberto/projects/aoc/2023/01.input").unwrap()
        );
    }
}
