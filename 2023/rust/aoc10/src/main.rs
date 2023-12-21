// every pipe connects to other two
// S is starting point, but has a pipe bellow
// S is 0
// from S try going in 4 directions
// from | down or up
// from - left or right
// from L up or  right
// from J up or left
// from 7 south or left
// from F south or right
// can't go to .
// map<position, distance>
// map[pos(S)] = 0
// if S try going 4 directions, exclude ground tiles (.)
// if different than S, try going possible directions according to pipe type
// possible tiles to visit are those non visited yet and no ground tiles
// if there is no more tile to visit, stop

use std::{collections::HashMap, fs};

type Diagram = Vec<Vec<char>>;

type Position = (usize, usize);

type VisitMap = HashMap<Position, usize>;

fn up(pos: &Position) -> Option<Position> {
    let x = pos.0;
    let y = pos.1;
    if x > 0 {
        return Some((x.wrapping_sub(1), y));
    }
    None
}

fn down(pos: &Position) -> Option<Position> {
    let x = pos.0;
    let y = pos.1;
    Some((x + 1, y))
}

fn left(pos: &Position) -> Option<Position> {
    let x = pos.0;
    let y = pos.1;
    if y > 0 {
        return Some((x, y.wrapping_sub(1)));
    }
    None
}

fn right(pos: &Position) -> Option<Position> {
    let x = pos.0;
    let y = pos.1;
    Some((x, y + 1))
}

fn find_starting_point(diagram: &Diagram) -> Result<(usize, usize), &'static str> {
    for (i, line) in diagram.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == 'S' {
                return Ok((i, j));
            }
        }
    }
    Err("Starting point not found")
}

fn parse_diagram(input: &str) -> Diagram {
    let mut diagram: Vec<Vec<char>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        diagram.push(Vec::new());
        for c in line.chars() {
            diagram[i].push(c);
        }
    }
    diagram
}

fn get_pipe(pos: &Position, diagram: &Diagram) -> char {
    diagram[pos.0][pos.1]
}

fn is_pipe(pos: &Position, diagram: &Diagram) -> bool {
    pos.1 < diagram[0].len() && diagram[pos.0][pos.1] != '.'
}

fn get_possible_directions(pos: &Position, diagram: &Diagram) -> Vec<Position> {
    let new_positions = match get_pipe(pos, diagram) {
        'S' => vec![up(pos), down(pos), left(pos), right(pos)],
        '|' => vec![up(pos), down(pos)],
        '-' => vec![right(pos), left(pos)],
        'L' => vec![up(pos), right(pos)],
        'J' => vec![up(pos), left(pos)],
        '7' => vec![down(pos), left(pos)],
        'F' => vec![down(pos), right(pos)],
        _ => vec![],
    };
    new_positions
        .into_iter()
        .filter(|opt_pos| match opt_pos {
            Some(p) => is_pipe(p, diagram),
            None => false,
        })
        .flatten()
        .collect()
}

fn visit_pipes(step: usize, current_pos: &Position, diagram: &Diagram, visited: &mut VisitMap) {
    // println!(
    //     "step = {}, current_pos = {:?}, visited = {:?}",
    //     step,
    //     get_pipe(current_pos, diagram),
    //     visited
    // );
    let num_steps = visited.entry((current_pos.0, current_pos.1)).or_default();
    if *num_steps > 0 && step > *num_steps {
        // println!(
        //     "already visited, step = {}, num_steps = {}",
        //     step, num_steps
        // );
        // already visited
        return;
    }
    *num_steps = step + 1;
    for pos in get_possible_directions(current_pos, diagram) {
        visit_pipes(step + 1, &pos, diagram, visited);
    }
}

fn main() {
    //     let example = r#"..F7.
    // .FJ|.
    // SJ.L7
    // |F--J
    // LJ..."#;
    let input = fs::read_to_string("/home/humberto/projects/aoc/2023/10.input").unwrap();
    let diagram = parse_diagram(&input);
    let start = find_starting_point(&diagram).unwrap();
    let mut visited: VisitMap = HashMap::new();
    visit_pipes(0, &start, &diagram, &mut visited);
    // get the farthest point
    let mut max = 0;
    let mut farthest = &start;
    for (pos, distance) in visited.iter() {
        if *distance > max {
            max = *distance;
            farthest = pos;
        }
    }
    println!(
        "farthest point is {:?} ({}) with distance {}",
        farthest,
        get_pipe(farthest, &diagram),
        max - 1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_starting_point() {
        let diagram = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        assert_eq!(
            (1, 1),
            find_starting_point(&parse_diagram(diagram)).unwrap()
        );
    }
}
