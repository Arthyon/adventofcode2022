use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

struct Sensor {
    beacon: Point,
    coordinates: Point,
    coverage_distance: i32,
}

fn main() {
    let path = "input";
    let items = read_file(path).map(parse_line).collect();
    let (x_min, x_max) = get_x_bounds(&items);

    let part1_line_no = if path == "test.txt" { 10 } else { 2000000 };

    let part1 = get_coverage(part1_line_no, x_min, x_max, &items);
    println!("{}", part1);

    let part2_line_no = part1_line_no * 2;

    let (x, y) = (0..part2_line_no)
        .find_map(|y| find_uncovered_position(y, &items))
        .unwrap();

    println!("{}", calculate_tuning_frequency(x, y));
}

fn calculate_tuning_frequency(x: i32, y: i32) -> i64 {
    i64::from(x) * 4000000i64 + i64::from(y)
}

fn get_x_bounds(items: &Vec<Sensor>) -> (i32, i32) {
    let x_coords: Vec<i32> = items
        .iter()
        .flat_map(|s| {
            vec![
                s.coordinates.x - s.coverage_distance,
                s.coordinates.x + s.coverage_distance,
            ]
        })
        .collect();
    (
        *x_coords.iter().min().unwrap(),
        *x_coords.iter().max().unwrap(),
    )
}

fn find_uncovered_position(line_no: i32, sensors: &Vec<Sensor>) -> Option<(i32, i32)> {
    let mut sensors: Vec<(i32, i32, i32)> = sensors
        .iter()
        .filter_map(|s| to_spectrum(line_no, s))
        .collect();

    sensors.sort_by_key(|(_, min_y, _)| *min_y);

    let mut max = 0;
    for (x, min_y, max_y) in sensors {
        if min_y <= max + 1 {
            max = std::cmp::max(max_y, max);
        } else {
            return Some((x, max + 1));
        }
    }
    None
}

fn to_spectrum(x: i32, sensor: &Sensor) -> Option<(i32, i32, i32)> {
    let Point { x: sx, y: sy } = sensor.coordinates;
    let distance = sensor.coverage_distance - (x - sx).abs();
    if distance < 0 {
        None
    } else {
        Some((x, sy - distance, sy + distance))
    }
}

fn get_coverage(line_no: i32, from_pos: i32, to_pos: i32, sensors: &Vec<Sensor>) -> i32 {
    let mut covered_positions = 0;
    for x in from_pos..=to_pos {
        let is_covered = sensors
            .iter()
            .any(|s| covers_empty_coordinate(x, line_no, s));
        if is_covered {
            covered_positions += 1;
        }
    }
    covered_positions
}

fn covers_empty_coordinate(x: i32, y: i32, sensor: &Sensor) -> bool {
    let length = manhattan_distance(&sensor.coordinates, x, y);
    let contains_beacon = x == sensor.beacon.x && y == sensor.beacon.y;
    if contains_beacon {
        false
    } else {
        contains_beacon || length <= sensor.coverage_distance
    }
}

fn manhattan_distance(Point { x: x1, y: y1 }: &Point, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_line(line: String) -> Sensor {
    let r = Regex::new(
        r"Sensor at x=([\d\-]+), y=([\d\-]+): closest beacon is at x=([\d\-]+), y=([\d\-]+)",
    )
    .unwrap();

    let captures = r.captures(&line).unwrap();
    let sensor = Point {
        x: captures.get(1).unwrap().as_str().parse().unwrap(),
        y: captures.get(2).unwrap().as_str().parse().unwrap(),
    };
    let beacon = Point {
        x: captures.get(3).unwrap().as_str().parse().unwrap(),
        y: captures.get(4).unwrap().as_str().parse().unwrap(),
    };

    let distance = manhattan_distance(&sensor, beacon.x, beacon.y);

    Sensor {
        beacon: beacon,
        coordinates: sensor,
        coverage_distance: distance,
    }
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().map(|l| l.unwrap())
}
