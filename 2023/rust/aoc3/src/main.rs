use regex::bytes::Regex;
use std::fs::{self};

fn part1() -> usize {
    let mut result = 0;
    let input = fs::read_to_string("/home/humberto/projects/aoc/2023/03.input").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"\b\d+\b").unwrap();

    //for each line test if number is surrounded by a symbol, if its add it to the result
    lines.iter().enumerate().for_each(|(i, line)| {
        // let number = line.replace('.', " ");
        re.find_iter(line.as_bytes()).for_each(|m| {
            let number_value = String::from_utf8_lossy(m.as_bytes())
                .parse::<usize>()
                .unwrap();
            for j in m.start()..m.end() {
                // Construct the surrounding array
                let surrounding = vec![
                    lines
                        .get(i.wrapping_sub(1))
                        .and_then(|row| row.chars().nth(j.wrapping_sub(1)))
                        .unwrap_or('.'),
                    lines
                        .get(i.wrapping_sub(1))
                        .and_then(|row| row.chars().nth(j))
                        .unwrap_or('.'),
                    lines
                        .get(i.wrapping_sub(1))
                        .and_then(|row| row.chars().nth(j + 1))
                        .unwrap_or('.'),
                    lines
                        .get(i)
                        .and_then(|row| row.chars().nth(j.wrapping_sub(1)))
                        .unwrap_or('.'),
                    lines
                        .get(i)
                        .and_then(|row| row.chars().nth(j))
                        .unwrap_or('.'),
                    lines
                        .get(i)
                        .and_then(|row| row.chars().nth(j + 1))
                        .unwrap_or('.'),
                    lines
                        .get(i + 1)
                        .and_then(|row| row.chars().nth(j.wrapping_sub(1)))
                        .unwrap_or('.'),
                    lines
                        .get(i + 1)
                        .and_then(|row| row.chars().nth(j))
                        .unwrap_or('.'),
                    lines
                        .get(i + 1)
                        .and_then(|row| row.chars().nth(j + 1))
                        .unwrap_or('.'),
                ];
                // test if the surrounding has a symbol
                if surrounding.iter().any(|c| !c.is_ascii_digit() && *c != '.') {
                    result += number_value;
                    break;
                }
            }
        });
    });
    result
}

fn main() {
    println!("{}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(556367, part1());
    }
}
