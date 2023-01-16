use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

type Point = (i32, i32, i32);

fn main() {
    let blocks: HashSet<_> = read_file("input").map(parse_line).collect();

    println!("{}", find_surface_area(&blocks));

    let bounds = get_bounding_box(&blocks);
    println!("{}", find_exposed_surface_area(bounds, &blocks));
}

fn find_surface_area(blocks: &HashSet<Point>) -> usize {
    let neighbours = blocks.iter().flat_map(get_neighbours);
    neighbours.filter(|n| !blocks.contains(n)).count()
}

fn get_neighbours((x, y, z): &Point) -> Vec<Point> {
    vec![
        (x + 1, *y, *z),
        (*x, y + 1, *z),
        (*x, *y, z + 1),
        (x - 1, *y, *z),
        (*x, y - 1, *z),
        (*x, *y, z - 1),
    ]
}

fn find_exposed_surface_area(bounds: (Point, Point), blocks: &HashSet<Point>) -> usize {
    let mut surface = HashSet::new();
    let mut air = HashSet::from([bounds.1]);

    let mut queue = VecDeque::from([bounds.1]);

    loop {
        match queue.pop_front() {
            None => {
                let neighbours = surface.iter().flat_map(get_neighbours);
                return neighbours.filter(|n| air.contains(n)).count();
            }
            Some(pos) => {
                let neighbours: HashSet<_> = get_neighbours(&pos)
                    .into_iter()
                    .filter(|n| in_bounds(bounds, n) && !air.contains(n))
                    .collect();

                surface.extend(neighbours.iter().filter(|n| blocks.contains(n)).map(|n| *n));

                let new_neighbours: HashSet<_> = neighbours.difference(&surface).collect();
                queue.extend(new_neighbours.clone());
                air.extend(new_neighbours);
            }
        }
    }
}

fn min_max(blocks: &HashSet<Point>, mapper: fn(&Point) -> &i32) -> (&i32, &i32) {
    let mapped: Vec<&i32> = blocks.iter().map(mapper).collect();
    (*mapped.iter().min().unwrap(), *mapped.iter().max().unwrap())
}

fn get_bounding_box(blocks: &HashSet<Point>) -> (Point, Point) {
    let (min_x, max_x) = min_max(blocks, |(x, _, _)| x);
    let (min_y, max_y) = min_max(blocks, |(_, y, _)| y);
    let (min_z, max_z) = min_max(blocks, |(_, _, z)| z);
    (
        (max_x + 1, max_y + 1, max_z + 1),
        (min_x - 1, min_y - 1, min_z - 1),
    )
}

fn in_bounds(
    ((max_x, max_y, max_z), (min_x, min_y, min_z)): (Point, Point),
    (x, y, z): &Point,
) -> bool {
    x <= &max_x && x >= &min_x && y <= &max_y && y >= &min_y && z <= &max_z && z >= &min_z
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().map(|l| l.unwrap())
}

fn parse_line(line: String) -> Point {
    let numbers: Vec<_> = line.split(",").map(|n| n.parse().unwrap()).collect();
    (numbers[0], numbers[1], numbers[2])
}
