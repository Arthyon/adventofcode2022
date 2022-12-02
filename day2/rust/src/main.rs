use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut sum_part1 = 0;
        let mut sum_part2 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let part1_score = calculate_part1(ip.as_ref());
                let part2_score = calculate_part2(ip.as_ref());
                sum_part1 += part1_score;
                sum_part2 += part2_score;
            }
        }
        println!("Part1: {}", sum_part1);
        println!("Part2: {}", sum_part2);
    }
}
fn calculate_part1(s: &str) -> i32 {
    match s {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("Wrong input"),
    }
}

// rock 1, paper 2, scissor 3
// win 6, draw 3, lose 0
// x = lose, y = draw, z = win
fn calculate_part2(s: &str) -> i32 {
    match s {
        "A X" => 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => panic!("Wrong input"),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
