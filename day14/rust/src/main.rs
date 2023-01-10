use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error, Lines},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points: HashSet<_> = read_stream("input").flat_map(parse_line).collect();

    let start = Point { x: 500, y: 0 };
    let floor = points.iter().map(|p| p.y).max().unwrap() + 2;

    let p1 = start_sandfall(&points, &start, floor, |p| p.y >= floor - 1);
    println!("{}", p1);
    let p2 = start_sandfall(&points, &start, floor, |x| x == &start);
    println!("{}", p2 + 1);
}

fn start_sandfall(
    points: &HashSet<Point>,
    start: &Point,
    floor: i32,
    should_stop: impl Fn(&Point) -> bool,
) -> i32 {
    let mut points = points.clone();
    let mut sand_amount = 0;
    loop {
        let new_pos = drop_sand(&points, start.clone(), floor);
        if should_stop(&new_pos) {
            return sand_amount;
        }
        sand_amount += 1;
        points.insert(new_pos);
    }
}

fn drop_sand(points: &HashSet<Point>, pos: Point, floor: i32) -> Point {
    let down = Point {
        x: pos.x,
        y: pos.y + 1,
    };
    let left = Point {
        x: pos.x - 1,
        y: pos.y + 1,
    };
    let right = Point {
        x: pos.x + 1,
        y: pos.y + 1,
    };
    if pos.y + 1 == floor {
        pos
    } else if !points.contains(&down) {
        drop_sand(points, down, floor)
    } else if !points.contains(&left) {
        drop_sand(points, left, floor)
    } else if !points.contains(&right) {
        drop_sand(points, right, floor)
    } else {
        pos
    }
}

fn parse_line(line: Result<String, Error>) -> Vec<Point> {
    let line = line.unwrap();
    let points: Vec<Point> = line.split("->").map(to_point).collect();
    points
        .windows(2)
        .flat_map(|window| create_range(&window[0], &window[1]))
        .collect()
}

fn create_range(from: &Point, to: &Point) -> Vec<Point> {
    let range = if from.x > to.x {
        to.x..=from.x
    } else if to.x > from.x {
        from.x..=to.x
    } else if from.y > to.y {
        to.y..=from.y
    } else {
        from.y..=to.y
    };

    if from.x != to.x {
        range.map(|x| Point { x: x, y: from.y }).collect()
    } else {
        range.map(|y| Point { x: from.x, y: y }).collect()
    }
}

fn to_point(s: &str) -> Point {
    let parts: Vec<_> = s.split(",").map(|s| s.trim()).collect();
    Point {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
    }
}

fn read_stream(path: &str) -> Lines<BufReader<File>> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines()
}
