use std::fs::{self};

fn cartesian_product<T: Clone>(vec1: &[T], vec2: &[T]) -> Vec<(T, T)> {
    vec1.iter()
        .flat_map(|x| vec2.iter().map(move |y| (x.to_owned(), y.to_owned())))
        .collect()
}

// return if number is adjacent to a symbol
fn is_part_number(number: &[(usize, usize)], symbol_positions: &[(usize, usize)]) -> bool {
    let adjacent_neighbors =
        cartesian_product(&[-1isize, 0isize, 1isize], &[-1isize, 0isize, 1isize]);
    for (row, column) in number {
        for (x, y) in &adjacent_neighbors {
            let i = x + *row as isize;
            let j = y + *column as isize;
            if i < 0 || j < 0 {
                continue;
            }
            if symbol_positions.contains(&(i as usize, j as usize)) {
                return true;
            }
        }
    }
    false
}

fn get_char_at_pos(text: &str, pos: &(usize, usize), row_len: usize) -> char {
    text.chars().nth(pos.1 + row_len * pos.0).unwrap()
}

// from the digit positions returns the number from the string
fn get_number(digits_positions: &[(usize, usize)], text: &str, line_len: usize) -> usize {
    let value = digits_positions
        .iter()
        .map(|pos| get_char_at_pos(text, pos, line_len))
        .collect::<String>();
    value
        .parse::<usize>()
        .expect(&("not a number ".to_owned() + &value))
}

fn part1_() -> usize {
    let mut result = 0;
    let input: Vec<&str> = fs::read_to_string("/home/humberto/projects/aoc/2023/03.input").unwrap()
        .split("\n").collect();
    for i in 0..input.len() {
        let numbers = input[i].replace(".", " ");
    }
    result
}

fn part1() -> usize {
    let input_string = fs::read_to_string("/home/humberto/projects/aoc/2023/03.input").unwrap();
    let mut consecutive_digit_positions: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut current_consecutive_positions: Vec<(usize, usize)> = Vec::new();
    let mut symbol_positions: Vec<(usize, usize)> = Vec::new();
    let row_len = input_string.find('\n').unwrap();
    let input_no_newlines = input_string.replace('\n', "");

    for (i, c) in input_no_newlines.trim().char_indices() {
        let row = i / row_len;
        let column = i % row_len;
        // capture digits positions
        if c.is_ascii_digit() {
            current_consecutive_positions.push((row, column));
        } else if !current_consecutive_positions.is_empty() {
            consecutive_digit_positions.push(current_consecutive_positions.clone());
            current_consecutive_positions.clear();
        }

        // capture symbol positions
        if c != '.' && !c.is_ascii_digit() && !c.is_whitespace() {
            symbol_positions.push((row, column));
        }
    }

    // Check if there are consecutive digits at the end of the string
    if !current_consecutive_positions.is_empty() {
        consecutive_digit_positions.push(current_consecutive_positions);
    }

    let sum: usize = consecutive_digit_positions
        .iter()
        .filter(|number| is_part_number(number, &symbol_positions))
        .map(|number| get_number(number, &input_no_newlines, row_len))
        .sum();
    sum
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
