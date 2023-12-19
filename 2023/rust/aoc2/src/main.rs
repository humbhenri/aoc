use std::fs;

#[derive(Debug, Default, PartialEq)]
struct Subset {
    greens: u32,
    blues: u32,
    reds: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

// returns true if all attributes of subset 1 is less or equal then subset 2
fn less(s1: &Subset, s2: &Subset) -> bool {
    s1.greens <= s2.greens && s1.blues <= s2.blues && s1.reds <= s2.reds
}

fn parse_subset(text: &str) -> Subset {
    let mut greens = 0;
    let mut blues = 0;
    let mut reds = 0;
    text.split(',')
        .map(|substr| {
            let parts: Vec<&str> = substr.split_whitespace().collect();
            let count = parts[0].parse::<u32>().expect("invalid number");
            let color = parts[1];
            (color, count)
        })
        .for_each(|(color, count)| {
            if color == "green" {
                greens = count;
            }
            if color == "blue" {
                blues = count;
            }
            if color == "red" {
                reds = count;
            }
        });
    Subset {
        greens,
        blues,
        reds,
    }
}

fn parse_game(line: &str) -> Game {
    let parts = &mut line.split(':');
    let game_header = parts.next().expect("Invalid line");
    let id = game_header
        .split_whitespace()
        .nth(1)
        .map(|x| x.parse::<u32>())
        .expect("invalid game id")
        .unwrap();
    let subsets = parts
        .next()
        .expect("invalid subsets")
        .split(';')
        .map(parse_subset)
        .collect::<Vec<Subset>>();

    Game { id, subsets }
}

fn is_possible(game: &Game, bag: &Subset) -> bool {
    game.subsets.iter().all(|subset| less(subset, bag))
}

fn part1(file: &str) -> u32 {
    let bag = Subset {
        reds: 12,
        greens: 13,
        blues: 14,
    };
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(parse_game)
        .filter(|game| is_possible(game, &bag))
        .map(|game| game.id)
        .sum()
}

fn part2(file: &str) -> u32 {
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(parse_game)
        .map(|game| {
            let mut max_green = 0;
            let mut max_blue = 0;
            let mut max_red = 0;
            game.subsets.iter().for_each(|subset| {
                max_green = max_green.max(subset.greens);
                max_blue = max_blue.max(subset.blues);
                max_red = max_red.max(subset.reds);
            });
            max_green * max_blue * max_red
        })
        .sum()
}

fn main() {
    println!(
        "part 1 = {}",
        part1("/home/humberto/projects/aoc/2023/02.input")
    );
    println!(
        "part 2 = {}",
        part2("/home/humberto/projects/aoc/2023/02.input")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_less() {
        assert!(less(
            &Subset {
                greens: 0,
                blues: 3,
                reds: 4
            },
            &Subset {
                greens: 13,
                blues: 14,
                reds: 12
            }
        ));
        assert!(!less(
            &Subset {
                greens: 8,
                blues: 6,
                reds: 20
            },
            &Subset {
                greens: 13,
                blues: 14,
                reds: 12
            }
        ));
    }

    #[test]
    fn test_parse_subset() {
        assert_eq!(
            Subset {
                blues: 3,
                reds: 4,
                greens: 0
            },
            parse_subset("3 blue, 4 red")
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            Game {
                id: 1,
                subsets: vec![
                    Subset {
                        blues: 3,
                        reds: 4,
                        greens: 0
                    },
                    Subset {
                        reds: 1,
                        greens: 2,
                        blues: 6
                    },
                    Subset {
                        greens: 2,
                        blues: 0,
                        reds: 0
                    }
                ]
            },
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
    }

    #[test]
    fn test_is_possible() {
        let game = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let bag = Subset {
            reds: 12,
            greens: 13,
            blues: 14,
        };
        assert!(is_possible(&game, &bag));
    }

    #[test]
    fn test_is_not_possible() {
        let game =
            parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        let bag = Subset {
            reds: 12,
            greens: 13,
            blues: 14,
        };
        assert!(!is_possible(&game, &bag));
    }
}
