use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    if let Ok(mut lines) = read_lines("./src/input.txt") {
        let mut amounts = Vec::new();
        loop {
            let result = sum_calories(&mut lines);
            if result == 0 {
                break;
            }
            amounts.push(result);
        }
        amounts.sort_unstable();
        amounts.reverse();
        let sum: u32 = amounts[..3].iter().sum();
        println!("Max amount for top three elves: {}", sum);
    }
}

fn sum_calories(lines: &mut io::Lines<io::BufReader<File>>) -> u32 {
    fn sum_internal(total: u32, lines: &mut io::Lines<io::BufReader<File>>) -> u32 {
        let line = lines.next();
        match line {
            None => total,
            Some(Ok(s)) => {
                if s == "" {
                    total
                } else {
                    let calories: u32 = s.parse().unwrap();
                    sum_internal(total + calories, lines)
                }
            }
            Some(Err(e)) => panic!("{}", e),
        }
    }
    sum_internal(0, lines)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
