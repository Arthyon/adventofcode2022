use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use jagged_array::Jagged2;

type Point = (i32, i32);

struct Map {
    rows: i32,
    cols: i32,
    start: Point,
    end: Point,
    map: Jagged2<char>,
}

impl Map {
    fn is_in_bounds(&self, (r, c): Point) -> bool {
        if self.start == (r, c) || self.end == (r, c) {
            true
        } else {
            r > 0 && r < self.rows && c > 0 && c < self.cols
        }
    }

    fn check_forward(idx: i32, time: &i32, size: i32) -> usize {
        let p = idx - ((time + 1) % size);
        if p < 1 {
            (p + size) as usize
        } else {
            p as usize
        }
    }

    fn check_backward(idx: i32, time: &i32, size: i32) -> usize {
        let p = idx + ((time + 1) % size);
        if p > size {
            (p - size) as usize
        } else {
            p as usize
        }
    }

    fn contains_wind(&self, (r, c): Point, time: &i32) -> bool {
        let r_forward = Map::check_forward(r, time, self.rows - 1);
        let r_backward = Map::check_backward(r, time, self.rows - 1);

        let c_forward = Map::check_forward(c, time, self.cols - 1);
        let c_backward = Map::check_backward(c, time, self.cols - 1);
        self.map.get([r_forward, c as usize]) == Some(&'v')
            || self.map.get([r_backward, c as usize]) == Some(&'^')
            || self.map.get([r as usize, c_forward]) == Some(&'>')
            || self.map.get([r as usize, c_backward]) == Some(&'<')
    }
}

fn main() {
    let map = parse_map(read_file("input"));

    let part_1 = shortest_path(&map, map.start, map.end, 0);
    println!("{}", part_1);

    let back = shortest_path(&map, map.end, map.start, part_1);
    let part_2 = shortest_path(&map, map.start, map.end, back);
    println!("{}", part_2);
}

fn shortest_path(map: &Map, from: Point, to: Point, time: i32) -> i32 {
    let mut visited = HashSet::from([(from, time)]);
    let mut queue = VecDeque::from([(from, time)]);

    loop {
        match queue.pop_front() {
            None => panic!("No available path!"),
            Some((pos, time)) => {
                if pos == to {
                    return time;
                }
                let neighbours: Vec<_> = get_potential_positions(map, pos, &time)
                    .filter(|v| !visited.contains(&(*v, time + 1)))
                    .collect();
                for n in neighbours {
                    visited.insert((n, time + 1));
                    queue.push_back((n, time + 1));
                }
            }
        }
    }
}

fn get_potential_positions<'a>(
    map: &'a Map,
    (r, c): Point,
    time: &'a i32,
) -> impl Iterator<Item = Point> + 'a {
    [(r, c), (r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
        .into_iter()
        .filter(|pos| map.is_in_bounds(*pos) && !map.contains_wind(*pos, time))
}

fn parse_map(map: Jagged2<char>) -> Map {
    let row_length = map.len() - 1;
    let col_length = map.get_row(0).unwrap().len() - 1;
    let find_pos = |r| {
        map.get_row(r)
            .unwrap()
            .iter()
            .position(|c| c == &'.')
            .unwrap()
    };

    let start = find_pos(0);
    let end = find_pos(row_length);
    let row_length = row_length as i32;

    Map {
        rows: row_length,
        cols: col_length as i32,
        start: (0, start as i32),
        end: (row_length, end as i32),
        map,
    }
}

fn read_file(path: &str) -> Jagged2<char> {
    let f = File::open(path).unwrap();
    Jagged2::from_iter(
        BufReader::new(f)
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>()),
    )
}
