use std::fs;

fn next_value(seq: &[i32]) -> i32 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }
    seq.last().unwrap() + next_value(&sequence_below(seq))
}

fn sequence_below(seq: &[i32]) -> Vec<i32> {
    if seq.iter().all(|&x| x == 0) {
        return Vec::from(seq);
    }
    let rest = &seq[1..];
    seq.iter()
        .zip(rest.iter())
        .map(|(a, b)| (b - a))
        .collect::<Vec<i32>>()
}

fn first_value(seq: &[i32]) -> i32 {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    }
    seq.first().unwrap() - first_value(&sequence_below(seq))
}

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn main() {
    let input_path = "/home/humberto/projects/aoc/2023/09.input";
    let input = &parse_input(input_path);
    let part1: i32 = input.iter().map(|seq| next_value(seq)).sum();
    println!("Part 1 = {}", part1);

    let part2: i32 = input.iter().map(|seq| first_value(seq)).sum();
    println!("Part 2 = {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_below() {
        assert_eq!(vec![1, 1], sequence_below(&[1, 2, 3]));
        assert_eq!(vec![3, 3, 3, 3, 3], sequence_below(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(vec![0, 0, 0, 0], sequence_below(&[0, 0, 0, 0]));
    }

    #[test]
    fn test_next_value() {
        assert_eq!(18, next_value(&[0, 3, 6, 9, 12, 15]));
        assert_eq!(0, next_value(&[0, 0, 0]));
        assert_eq!(28, next_value(&[1, 3, 6, 10, 15, 21]));
        assert_eq!(68, next_value(&[10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn test_first_value() {
        assert_eq!(0, first_value(&[0, 0]));
        assert_eq!(2, first_value(&[2, 2, 2]));
        assert_eq!(-2, first_value(&[0, 2, 4, 6]));
        assert_eq!(5, first_value(&[10, 13, 16, 21, 30, 45]));
    }
}
