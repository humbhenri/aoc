// https://adventofcode.com/2023/day/8

use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

type Network = HashMap<String, (String, String)>;

fn parse_line(line: String, network: &mut Network) {
    let pattern = Regex::new(r"\b\w+\b").unwrap();
    let mut words = pattern.captures_iter(line.as_str());
    network.insert(
        words.next().unwrap().get(0).unwrap().as_str().to_owned(),
        (
            words.next().unwrap().get(0).unwrap().as_str().to_owned(),
            words.next().unwrap().get(0).unwrap().as_str().to_owned(),
        ),
    );
}

fn parse_input(file_name: String) -> (String, Network) {
    let file = File::open(file_name).unwrap();
    let reader = io::BufReader::new(file);
    let mut network: Network = HashMap::new();
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().unwrap();
    lines.next(); // empty line
    for line in lines {
        parse_line(line.unwrap(), &mut network);
    }
    (instructions, network)
}

fn count_steps(network: Network, instructions_line: String) -> u32 {
    let mut instructions = instructions_line.chars().cycle();
    let mut current = "AAA";
    let mut steps = 0;
    while current != "ZZZ" {
        steps += 1;
        let next = match instructions.next().unwrap() {
            'R' => network.get(current).map(|lr| &lr.1),
            'L' => network.get(current).map(|lr| &lr.0),
            _ => unreachable!(),
        };
        current = next.expect("invalid");
    }
    steps
}

fn part1() {
    let (instructions, network) =
        parse_input("/home/humberto/projects/aoc/2023/08.input".to_owned());
    println!("steps = {}", count_steps(network, instructions));
}

fn main() {
    part1()
}
