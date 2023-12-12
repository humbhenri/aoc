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

fn test() {
    let mut network: Network = HashMap::new();
    network.insert("AAA".to_owned(), ("BBB".to_owned(), "CCC".to_owned()));
    network.insert("BBB".to_owned(), ("DDD".to_owned(), "EEE".to_owned()));
    network.insert("CCC".to_owned(), ("ZZZ".to_owned(), "GGG".to_owned()));
    network.insert("DDD".to_owned(), ("DDD".to_owned(), "DDD".to_owned()));
    network.insert("EEE".to_owned(), ("EEE".to_owned(), "EEE".to_owned()));
    network.insert("GGG".to_owned(), ("GGG".to_owned(), "GGG".to_owned()));
    network.insert("ZZZ".to_owned(), ("ZZZ".to_owned(), "ZZZ".to_owned()));
    let mut instructions = ['R', 'R'].iter().cycle();
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
    println!("steps = {}", steps);
}

fn main() {
    let (instructions, network) =
        parse_input("/home/humberto/projects/aoc/2023/08.example".to_owned());
    println!("input = {:?}", network);
}
