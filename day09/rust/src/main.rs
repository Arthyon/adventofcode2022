use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let commands: Vec<String> = read_file("input")
        .flat_map(|l| to_commands(l.unwrap()))
        .collect();
    move_rope(&commands, 2);
    move_rope(&commands, 10);
}

fn move_rope(commands: &Vec<String>, rope_length: usize) {
    let mut visited = HashSet::new();
    let initial_position = (0, 0);
    visited.insert(initial_position);
    let mut rope: Vec<(i32, i32)> = std::iter::repeat(initial_position)
        .take(rope_length)
        .collect();

    for command in commands {
        rope[0] = move_head(rope[0], command);
        for i in 1..rope_length {
            rope[i] = follow(rope[i], rope[i - 1]);
        }
        visited.insert(*rope.last().unwrap());
    }

    println!("{}", visited.len());
}

fn move_head((x, y): (i32, i32), command: &String) -> (i32, i32) {
    match command.as_str() {
        "R" => (x + 1, y),
        "L" => (x - 1, y),
        "U" => (x, y + 1),
        "D" => (x, y - 1),
        _ => panic!("Unknown direction"),
    }
}

fn follow((x_pos, y_pos): (i32, i32), (x_target, y_target): (i32, i32)) -> (i32, i32) {
    let x_diff = x_target - x_pos;
    let y_diff = y_target - y_pos;
    match (x_diff.abs(), y_diff.abs()) {
        (0, 0) | (0, 1) | (1, 0) | (1, 1) => (x_pos, y_pos),
        (1, _) => (x_target, y_pos + y_diff.signum()),
        (_, 1) => (x_pos + x_diff.signum(), y_target),
        (_, _) => (x_pos + x_diff.signum(), y_pos + y_diff.signum()),
    }
}

fn to_commands(line: String) -> Vec<String> {
    let parts: Vec<&str> = line.split(" ").collect();
    std::iter::repeat(parts[0].to_string())
        .take(parts[1].parse().unwrap())
        .collect()
}

fn read_file(path: &str) -> std::io::Lines<BufReader<File>> {
    let l = File::open(path).unwrap();
    BufReader::new(l).lines()
}
