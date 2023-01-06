use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug)]
enum Command {
    Noop,
    Add(i32),
}

fn main() {
    let lines = read_lines("input").flat_map(parse_command).collect();
    start_cycles(&lines);
}

fn start_cycles(commands: &Vec<Command>) {
    let mut register = 1;
    let mut cumulative_strength = 0;
    let signals = vec![20, 60, 100, 140, 180, 220];
    for i in 0..commands.len() {
        if signals.contains(&(i + 1)) {
            cumulative_strength = cumulative_strength + calculate_signal_strength(i + 1, register)
        }

        draw_pixel(i, register);

        let cmd = &commands[i];
        if let Command::Add(val) = cmd {
            register = register + val;
        }
    }
    println!("");

    println!("{}", cumulative_strength);
}

fn draw_pixel(cycle: usize, register: i32) {
    let cycle: i32 = usize::try_into(cycle % 40).unwrap();
    if cycle == 0 {
        println!("");
    }
    if vec![register - 1, register, register + 1].contains(&cycle) {
        print!("#");
    } else {
        print!(" ");
    }
}

fn calculate_signal_strength(cycle: usize, register: i32) -> i32 {
    let cycle = i32::try_from(cycle).unwrap();
    cycle * register
}

fn parse_command(s: Result<String, std::io::Error>) -> Vec<Command> {
    let s = s.unwrap();
    match s.chars().next() {
        Some('n') => vec![Command::Noop],
        Some('a') => {
            let parts: Vec<&str> = s.split(" ").collect();
            vec![Command::Noop, Command::Add(parts[1].parse().unwrap())]
        }
        _ => panic!("unknown command"),
    }
}

fn read_lines(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}
