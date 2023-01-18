use core::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use jagged_array::Jagged2;
use regex::Regex;

#[derive(Debug)]
enum Movement {
    Walk(i32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Walkable,
    Wall,
    Void,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match &self {
            Block::Void => ' ',
            Block::Walkable => '.',
            Block::Wall => '#',
        };
        write!(f, "{}", c)
    }
}

type Point = (i32, i32);
type Direction = char;

#[derive(Debug, Clone)]
struct Player {
    pos: Point,
    dir: char,
}

#[derive(Debug)]
struct CubeFace {
    top: Point,
    size: i32,
    right: (Point, char, bool),
    left: (Point, char, bool),
    up: (Point, char, bool),
    down: (Point, char, bool),
}

enum WrappingMode {
    Normal,
    Cube(Vec<CubeFace>),
}

const LEFT: Direction = '<';
const UP: Direction = '^';
const DOWN: Direction = 'v';
const RIGHT: Direction = '>';
static DIRECTIONS: [Direction; 4] = [RIGHT, DOWN, LEFT, UP];

fn main() {
    let path = "input";
    let mut it = read_file(path);
    let map = build_map(&mut it);
    let movement = parse_movement(it);

    let start = map
        .get_row(0)
        .unwrap()
        .iter()
        .position(|b| *b == Block::Walkable)
        .unwrap();

    let player = Player {
        dir: RIGHT,
        pos: (0, start as i32),
    };

    let player1 = follow_path(&map, &movement, player.clone(), &WrappingMode::Normal);
    print_password(&player1);

    let faces = if path == "test.txt" {
        get_test_faces()
    } else {
        get_real_faces()
    };

    let player2 = follow_path(&map, &movement, player.clone(), &WrappingMode::Cube(faces));
    print_password(&player2);
}

fn print_password(player: &Player) {
    let (r, c) = player.pos;

    let idx = DIRECTIONS.iter().position(|d| d == &player.dir).unwrap() as i32;

    println!("{}", (r + 1) * 1000 + (c + 1) * 4 + idx);
}

fn follow_path(
    map: &Jagged2<Block>,
    movement: &Vec<Movement>,
    mut player: Player,
    mode: &WrappingMode,
) -> Player {
    for movement in movement {
        match movement {
            Movement::TurnLeft | Movement::TurnRight => turn_player(&mut player, movement),
            Movement::Walk(steps) => move_player(map, &mut player, steps, mode),
        }
    }

    player
}

fn turn_player(player: &mut Player, movement: &Movement) {
    let m = match movement {
        Movement::TurnLeft => -1,
        Movement::TurnRight => 1,
        _ => panic!("Cannot turn that way"),
    };
    let idx = DIRECTIONS.iter().position(|d| d == &player.dir).unwrap() as i32;
    player.dir = DIRECTIONS[(idx + m).rem_euclid(4) as usize];
}

fn move_player(map: &Jagged2<Block>, player: &mut Player, steps: &i32, mode: &WrappingMode) {
    for _ in 0..*steps {
        let movement = get_movement_direction(player.dir);
        let new_position = apply_movement(&player.pos, &movement);
        let next_block = get_block(map, new_position);
        match next_block {
            Block::Wall => break,
            Block::Walkable => player.pos = new_position,
            Block::Void => {
                let (new_pos, new_dir) = match mode {
                    WrappingMode::Normal => wrap_normal(map, player),
                    WrappingMode::Cube(face) => wrap_cube(map, player, face),
                };
                if new_pos == player.pos {
                    // encountered wall. double check if this will be correct
                    break;
                }
                player.dir = new_dir;
                player.pos = new_pos;
            }
        }
    }
}

fn apply_movement((x1, y1): &Point, (x2, y2): &Point) -> Point {
    (x1 + x2, y1 + y2)
}

fn get_movement_direction(dir: Direction) -> Point {
    match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("invalid dir"),
    }
}

fn get_block(map: &Jagged2<Block>, (r, c): Point) -> &Block {
    match map.get([r as usize, c as usize]) {
        Some(block) => block,
        None => &Block::Void,
    }
}

fn wrap_normal(map: &Jagged2<Block>, player: &Player) -> (Point, Direction) {
    let (r, c) = player.pos;
    let (r, c): (usize, usize) = match player.dir {
        '^' => {
            let r = (0..map.len())
                .rev()
                .find_map(|r| {
                    let block = map.get([r, c as usize]);
                    if block != Some(&Block::Void) && block != None {
                        Some(r)
                    } else {
                        None
                    }
                })
                .unwrap();
            (r, c as usize)
        }
        '>' => {
            let row = map.get_row(r as usize).unwrap();
            let (c, _) = row
                .iter()
                .enumerate()
                .find(|(_, block)| block != &&Block::Void)
                .unwrap();
            (r as usize, c)
        }
        'v' => {
            let r = (0..map.len())
                .find_map(|r| {
                    let block = map.get([r, c as usize]);
                    if block != Some(&Block::Void) && block != None {
                        Some(r)
                    } else {
                        None
                    }
                })
                .unwrap();
            (r, c as usize)
        }
        '<' => {
            let row = map.get_row(r as usize).unwrap();
            let (c, _) = row
                .iter()
                .enumerate()
                .rev()
                .find(|(_, block)| block != &&Block::Void)
                .unwrap();
            (r as usize, c)
        }
        _ => panic!("Invalid direction"),
    };

    if map.get([r, c]) == Some(&Block::Wall) {
        (player.pos, player.dir)
    } else {
        ((r as i32, c as i32), player.dir)
    }
}

fn wrap_cube(map: &Jagged2<Block>, player: &Player, faces: &Vec<CubeFace>) -> (Point, Direction) {
    let face = get_current_face(faces, player.pos);
    let ((r, c), next_dir, inverted) = next_face(face, player.dir);

    let x_delta = player.pos.0 - face.top.0;
    let y_delta = player.pos.1 - face.top.1;
    let (x_delta, y_delta) = get_face_delta(
        (x_delta, y_delta),
        face.size,
        inverted,
        (player.dir, next_dir),
    );

    let new_pos = (r + x_delta, c + y_delta);

    match get_block(map, new_pos) {
        Block::Wall => (player.pos, player.dir),
        Block::Walkable => (new_pos, next_dir),
        Block::Void => panic!("Should not be possible to encounter void"),
    }
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().map(|l| l.unwrap())
}

fn build_map(it: &mut impl Iterator<Item = String>) -> Jagged2<Block> {
    it.by_ref()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Block::Walkable,
                    '#' => Block::Wall,
                    ' ' => Block::Void,
                    _ => panic!("Invalid"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn parse_movement(it: impl Iterator<Item = String>) -> Vec<Movement> {
    let r = Regex::new(r"(\d+)([RL]?)*").unwrap();

    let mut movement = Vec::new();

    let dirs: String = it.filter(|l| !l.trim().is_empty()).next().unwrap();
    for m in r.captures_iter(&dirs) {
        movement.push(Movement::Walk(m.get(1).unwrap().as_str().parse().unwrap()));
        match m.get(2) {
            Some(v) => {
                if v.as_str() == "R" {
                    movement.push(Movement::TurnRight)
                } else {
                    movement.push(Movement::TurnLeft)
                }
            }
            _ => (),
        }
    }

    movement
}

fn get_face_delta(
    (delta_x, delta_y): Point,
    size: i32,
    inverted: bool,
    direction_change: (char, char),
) -> Point {
    match direction_change {
        ('<', '<') => (delta_x, size),
        ('>', '>') if inverted => (size - delta_x, size),
        ('>', '>') => (delta_x, 0),
        ('^', '^') if inverted => (size, delta_y),
        ('^', '^') => (size, delta_y),
        ('v', 'v') => (0, delta_y),
        ('<', '^') => (size, size - delta_x),
        ('<', 'v') => (0, delta_x),
        ('>', 'v') => (0, size - delta_x),
        ('>', '<') => (size - delta_x, size),
        ('^', '>') => (delta_y, 0),
        ('^', '<') => (size - delta_y, size),
        ('^', 'v') => (0, size - delta_y),
        ('v', '>') => (size - delta_y, 0),
        ('v', '^') => (size, size - delta_y),
        ('<', '>') => (size - delta_x, 0),
        ('>', '^') => (size, delta_x),
        ('v', '<') => (delta_y, size),
        _ => panic!("change not implemented"),
    }
}

fn next_face(face: &CubeFace, dir: char) -> (Point, char, bool) {
    match dir {
        '>' => face.right,
        '^' => face.up,
        '<' => face.left,
        'v' => face.down,
        _ => panic!("invalid dir"),
    }
}

fn get_current_face(faces: &Vec<CubeFace>, (x, y): Point) -> &CubeFace {
    faces
        .iter()
        .find(|f| {
            let (left, top) = f.top;
            let right = left + f.size;
            let bottom = top + f.size;
            x >= left && x <= right && y >= top && y <= bottom
        })
        .unwrap()
}

fn get_test_faces() -> Vec<CubeFace> {
    let (one, two, three, four, five, six) = ((0, 8), (4, 0), (4, 4), (4, 8), (8, 8), (8, 12));
    let test_size = 3; // one less than face size
    vec![
        CubeFace {
            top: one,
            size: test_size,
            right: (six, 'v', false),
            left: (three, 'v', false),
            up: (two, 'v', false),
            down: (four, 'v', false),
        }, // 1
        CubeFace {
            top: two,
            size: test_size,
            right: (three, '>', false),
            left: (six, '<', false),
            up: (one, 'v', false),
            down: (five, '^', false),
        }, // 2
        CubeFace {
            top: three,
            size: test_size,
            right: (four, '>', false),
            left: (two, '<', false),
            up: (one, '>', false),
            down: (five, '>', false),
        }, // 3
        CubeFace {
            top: four,
            size: test_size,
            right: (six, 'v', false),
            left: (three, '<', false),
            up: (one, '^', false),
            down: (five, 'v', false),
        }, // 4
        CubeFace {
            top: five,
            size: test_size,
            right: (six, '>', false),
            left: (three, '^', false),
            up: (four, '^', false),
            down: (two, '^', false),
        }, // 5
        CubeFace {
            top: six,
            size: test_size,
            right: (one, '<', false),
            left: (five, '<', false),
            up: (four, '<', false),
            down: (two, '>', false),
        }, // 6
    ]
}

fn get_real_faces() -> Vec<CubeFace> {
    let (one, two, three, four, five, six) =
        ((0, 50), (0, 100), (50, 50), (100, 0), (100, 50), (150, 0));
    let size = 49; // one less than face size
    vec![
        CubeFace {
            top: one,
            size: size,
            right: (two, '>', false),
            left: (four, '>', false),
            up: (six, '>', false),
            down: (three, 'v', false),
        }, // 1
        CubeFace {
            top: two,
            size: size,
            right: (five, '<', true),
            left: (one, '<', false),
            up: (six, '^', true),
            down: (three, '<', false),
        }, // 2
        CubeFace {
            top: three,
            size: size,
            right: (two, '^', false),
            left: (four, 'v', false),
            up: (one, '^', false),
            down: (five, 'v', false),
        }, // 3
        CubeFace {
            top: four,
            size: size,
            right: (five, '>', false),
            left: (one, '>', false),
            up: (three, '>', false),
            down: (six, 'v', false),
        }, // 4
        CubeFace {
            top: five,
            size: size,
            right: (two, '<', false),
            left: (four, '<', false),
            up: (three, '^', false),
            down: (six, '<', false),
        }, // 5
        CubeFace {
            top: six,
            size: size,
            right: (five, '^', false),
            left: (one, 'v', false),
            up: (four, '^', false),
            down: (two, 'v', false),
        }, // 6
    ]
}
