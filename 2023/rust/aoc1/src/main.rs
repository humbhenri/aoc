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
    let mut total = 0;
    for line in reader.lines() {
        let (first_digit, last_digit) = get_first_and_last_digit(line?);
        total += first_digit * 10 + last_digit;
    }
    Ok(total)
}

fn get_first_and_last_digit(line: String) -> (u32, u32) {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap();
    (first, digits.last().unwrap_or(first))
}

#[cfg(test)]
mod tests {
    use super::*;

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
