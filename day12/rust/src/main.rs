use jagged_array::Jagged2;
use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let map = create_map("input");
    let start = find_coordinate(&map, 'S');
    let end = find_coordinate(&map, 'E');

    let part1 = shortest_path(&map, start, 'E', false);
    println!("{}", part1);
    let part2 = shortest_path(&map, end, 'a', true);
    println!("{}", part2);
}

fn shortest_path(map: &Jagged2<char>, from: (usize, usize), to: char, rev: bool) -> i32 {
    let mut visited = HashSet::from([from]);
    let mut queue = VecDeque::from([(from, 0)]);

    loop {
        match queue.pop_front() {
            None => panic!("No available path!"),
            Some(((x, y), p)) => {
                if map.get([x, y]).unwrap() == &to {
                    return p;
                }
                let neighbours: Vec<_> = get_neighbours(map, (x, y), rev)
                    .filter(|v| !visited.contains(v))
                    .collect();
                for n in neighbours {
                    visited.insert(n);
                    queue.push_back((n, p + 1));
                }
            }
        }
    }
}

fn get_neighbours(
    map: &Jagged2<char>,
    (x, y): (usize, usize),
    rev: bool,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    let current_height = to_height(*map.get([x, y]).unwrap());
    let mut neighbours = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    neighbours
        .into_iter()
        .filter(move |pos| is_walkable(map, current_height, *pos, rev))
}

fn is_walkable(
    map: &Jagged2<char>,
    current_height: i32,
    (x, y): (usize, usize),
    rev: bool,
) -> bool {
    match map.get([x, y]) {
        None => false,
        Some(val) if rev => to_height(*val) >= current_height - 1,
        Some(val) => to_height(*val) <= current_height + 1,
    }
}

fn create_map(path: &str) -> Jagged2<char> {
    let l = fs::read_to_string(path).unwrap();
    let l: Vec<Vec<char>> = l.split("\n").map(|l| l.chars().collect()).collect();
    Jagged2::from_iter(l)
}

fn find_coordinate(map: &Jagged2<char>, c: char) -> (usize, usize) {
    for row in 0..map.len() {
        let r = map.get_row(row).unwrap();
        for col in 0..r.len() {
            if map.get([row, col]).unwrap() == &c {
                return (row, col);
            }
        }
    }
    panic!("not found");
}

fn to_height(c: char) -> i32 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => c as i32 - 97,
    }
}
