use regex::bytes::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::{self},
};

type Location = (usize, usize);
type NumberLocation = (usize, usize, usize); // row, column, number value

fn neighbors_coordinates(x: usize, y: usize) -> Vec<Location> {
    let min: i32 = -1;
    let max: i32 = 1;
    let mut result: Vec<(usize, usize)> = Vec::new();
    for i in min..=max {
        for j in min..=max {
            let dx: i32 = x as i32 + i;
            let dy: i32 = y as i32 + j;
            if dx < 0 || dy < 0 {
                continue;
            }
            result.push((dx as usize, dy as usize));
        }
    }
    result
}

fn main() {
    let mut part1 = 0;
    let input = fs::read_to_string("/home/humberto/projects/aoc/2023/03.input").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"\b\d+\b").unwrap();
    // key is star location, value is number locations and value adjacent to stars
    let mut stars: HashMap<Location, HashSet<NumberLocation>> = HashMap::new();

    //for each line test if number is surrounded by a symbol, if its add it to the result
    lines.iter().enumerate().for_each(|(i, line)| {
        re.find_iter(line.as_bytes()).for_each(|m| {
            let number_value = String::from_utf8_lossy(m.as_bytes())
                .parse::<usize>()
                .unwrap();

            // m.start is the first column of the number, m.end the last
            for j in m.start()..m.end() {
                // surrounding symbols of location (i, j), is is the current row and j the column
                let surrounding = neighbors_coordinates(i, j)
                    .iter()
                    .map(|&(x, y)| {
                        let symbol = lines
                            .get(x)
                            .and_then(|row| row.chars().nth(y))
                            .unwrap_or('.');
                        (x, y, symbol)
                    })
                    .collect::<Vec<(usize, usize, char)>>();

                // part 2 - collect stars and his adjacent numbers
                surrounding
                    .iter()
                    .filter(|(_, _, c)| *c == '*')
                    .for_each(|(x, y, _)| {
                        stars.entry((*x, *y)).or_insert(HashSet::new()).insert((
                            i,
                            j,
                            number_value,
                        ));
                    });

                // part 1: test if the surrounding has a symbol
                if surrounding
                    .iter()
                    .any(|(_, _, c)| !c.is_ascii_digit() && *c != '.')
                {
                    part1 += number_value;
                    break;
                }
            }
        });
    });

    // sum of the gears, a gear is the product of adjacent numbers of a star and a gear
    // must have exactly two adjacent numbers
    let part2: usize = stars
        .iter()
        .filter(|&(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers.iter().fold(1, |acc, (_, _, value)| acc * value))
        .sum::<usize>();

    println!("part 1 = {}, part 2 = {}", part1, part2);
}
