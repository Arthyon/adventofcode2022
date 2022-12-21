use array2d::Array2D;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let lines = read_lines("input");
    let char_vec: Vec<Vec<u32>> = lines.iter().map(string_to_number_vec).collect();
    let forest = Array2D::from_rows(&char_vec).expect("Could create 2d array");

    let mut visible_trees = 0;
    let mut scenic_score = 0;

    for x in 0..forest.num_rows() {
        for y in 0..forest.num_columns() {
            let score = get_scenic_score(&forest, x, y);
            if score > scenic_score {
                scenic_score = score;
            }
            if is_visible(&forest, x, y) {
                visible_trees = visible_trees + 1;
            }
        }
    }
    println!("{}", visible_trees);
    println!("{}", scenic_score);
}

fn get_scenic_score(forest: &Array2D<u32>, x: usize, y: usize) -> usize {
    let mut vertical = forest.column_iter(y).expect("Cannot iterate");
    let mut horizontal = forest.row_iter(x).expect("Cannot iterate");
    let (north, south) = split_direction(&mut vertical, x);
    let (west, east) = split_direction(&mut horizontal, y);
    let value = forest[(x, y)];

    let north_distance = get_view_distance(north, value);
    let south_distance = get_view_distance(south, value);
    let west_distance = get_view_distance(west, value);
    let east_distance = get_view_distance(east, value);

    north_distance * south_distance * west_distance * east_distance
}

fn get_view_distance(dir: Vec<&u32>, tree_height: u32) -> usize {
    let original_length = dir.len();
    let distance = dir.into_iter().take_while(|v| **v < tree_height).count();
    if distance == original_length {
        distance
    } else {
        distance + 1
    }
}

fn is_visible(forest: &Array2D<u32>, x: usize, y: usize) -> bool {
    let mut vertical = forest.column_iter(y).expect("Cannot iterate");
    let mut horizontal = forest.row_iter(x).expect("Cannot iterate");
    let (north, south) = split_direction(&mut vertical, x);
    let (west, east) = split_direction(&mut horizontal, y);

    let value = forest[(x, y)];
    let north_visibility = north.into_iter().all(|v| *v < value);
    let south_visibility = south.into_iter().all(|v| *v < value);
    let west_visibility = west.into_iter().all(|v| *v < value);
    let east_visibility = east.into_iter().all(|v| *v < value);
    north_visibility || south_visibility || west_visibility || east_visibility
}

fn split_direction<'a>(
    it: &mut impl DoubleEndedIterator<Item = &'a u32>,
    at_index: usize,
) -> (Vec<&'a u32>, Vec<&'a u32>) {
    let mut dir1: Vec<&u32> = Vec::new();
    let mut dir2: Vec<&u32> = Vec::new();

    let mut i = 0;
    for v in it {
        if i < (at_index) {
            dir1.push(v);
        } else if i > (at_index) {
            dir2.push(v);
        }
        i = i + 1;
    }
    dir1.reverse();
    (dir1, dir2)
}

fn string_to_number_vec(s: &String) -> Vec<u32> {
    s.chars()
        .map(|c| c.to_digit(10).expect("Cannot parse digit"))
        .collect()
}

fn read_lines(file: &str) -> Vec<String> {
    let file = File::open(file).expect("Not such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
