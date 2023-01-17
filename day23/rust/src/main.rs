use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use array2d::Array2D;

type Point = (i32, i32);

enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}
impl Direction {
    fn move_from(&self, (x, y): &Point) -> Point {
        match *self {
            Direction::N => (x - 1, *y),
            Direction::S => (x + 1, *y),
            Direction::E => (*x, y + 1),
            Direction::W => (*x, y - 1),
            Direction::NE => (x - 1, y + 1),
            Direction::NW => (x - 1, y - 1),
            Direction::SE => (x + 1, y + 1),
            Direction::SW => (x + 1, y - 1),
        }
    }
}

static MOVEMENT_GROUPS: [[Direction; 3]; 4] = [
    [Direction::N, Direction::NE, Direction::NW],
    [Direction::S, Direction::SE, Direction::SW],
    [Direction::W, Direction::NW, Direction::SW],
    [Direction::E, Direction::NE, Direction::SE],
];

fn main() {
    let positions = read_file("input");
    let part1_positions = perform_rounds(positions.clone(), 10);
    println!("{}", calculate_empty_tiles(&part1_positions));

    let rounds_needed = move_until_not_needed(positions);
    println!("{}", rounds_needed);
}

fn perform_rounds(mut positions: HashSet<Point>, rounds: i32) -> HashSet<(i32, i32)> {
    let mut moveset = 0;
    for _ in 0..rounds {
        let (moves, next_moveset) = propose_moves(moveset, &positions);
        moveset = next_moveset;
        move_elves(&mut positions, moves);
    }
    positions
}

fn move_until_not_needed(mut positions: HashSet<Point>) -> i32 {
    let mut moveset = 0;
    let mut rounds = 1;
    loop {
        let (moves, next_moveset) = propose_moves(moveset, &positions);
        if moves.len() == 0 {
            return rounds;
        }
        moveset = next_moveset;
        move_elves(&mut positions, moves);
        rounds += 1;
    }
}

fn propose_moves(moveset: usize, positions: &HashSet<Point>) -> (Vec<(Point, Point)>, usize) {
    let mut potential_moves = Vec::new();
    for position in positions {
        if !is_vacant(
            position,
            positions,
            MOVEMENT_GROUPS.iter().flatten().collect(),
        ) {
            if is_vacant(
                position,
                positions,
                MOVEMENT_GROUPS[moveset].iter().collect(),
            ) {
                potential_moves.push((
                    position.clone(),
                    MOVEMENT_GROUPS[moveset][0].move_from(position),
                ));
            } else if is_vacant(
                position,
                positions,
                MOVEMENT_GROUPS[(moveset + 1) % 4].iter().collect(),
            ) {
                potential_moves.push((
                    position.clone(),
                    MOVEMENT_GROUPS[(moveset + 1) % 4][0].move_from(position),
                ));
            } else if is_vacant(
                position,
                positions,
                MOVEMENT_GROUPS[(moveset + 2) % 4].iter().collect(),
            ) {
                potential_moves.push((
                    position.clone(),
                    MOVEMENT_GROUPS[(moveset + 2) % 4][0].move_from(position),
                ));
            } else if is_vacant(
                position,
                positions,
                MOVEMENT_GROUPS[(moveset + 3) % 4].iter().collect(),
            ) {
                potential_moves.push((
                    position.clone(),
                    MOVEMENT_GROUPS[(moveset + 3) % 4][0].move_from(position),
                ));
            }
        }
    }
    (potential_moves, (moveset + 1) % 4)
}

fn move_elves(positions: &mut HashSet<Point>, moves: Vec<(Point, Point)>) {
    // Better way to find unique values?
    let s: HashSet<_> = moves
        .iter()
        .fold(HashMap::new(), |mut acc, (_, to)| {
            if let Some(count) = acc.get(to) {
                acc.insert(*to, count + 1);
            } else {
                acc.insert(*to, 1);
            }
            acc
        })
        .iter()
        .filter_map(|(pos, count)| if *count == 1 { Some(pos.clone()) } else { None })
        .collect();

    for (from, to) in moves {
        if s.contains(&to) && positions.remove(&from) {
            positions.insert(to);
        }
    }
}

fn is_vacant(position: &Point, positions: &HashSet<Point>, directions: Vec<&Direction>) -> bool {
    let not_empty = directions
        .iter()
        .map(|d| d.move_from(position))
        .any(|pos| positions.contains(&pos));
    !not_empty
}

fn calculate_empty_tiles(positions: &HashSet<Point>) -> i32 {
    let min_x = positions.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = positions.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = positions.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = positions.iter().max_by_key(|(_, y)| y).unwrap().1;
    let area = (max_x - min_x + 1) * (max_y - min_y + 1);

    area - i32::try_from(positions.len()).unwrap()
}

fn read_file(path: &str) -> HashSet<Point> {
    let f = File::open(path).unwrap();
    let lines: Vec<Vec<_>> = BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let array = Array2D::from_rows(&lines).unwrap();

    array
        .indices_row_major()
        .filter_map(|(x, y)| {
            if let Some('#') = array.get(x, y) {
                Some((i32::try_from(x).unwrap(), i32::try_from(y).unwrap()))
            } else {
                None
            }
        })
        .collect()
}
