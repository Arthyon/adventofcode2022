use std::{collections::HashSet, fs};

fn main() {
    if let Ok(input) = fs::read_to_string("input") {
        let chars: Vec<char> = input.chars().collect();
        let part1 = find_marker(4, &chars);
        print!("part1: {}\n", part1);
        let part2 = find_marker(14, &chars);
        print!("part2: {}", part2);
    }
}

fn find_marker(lookahead: usize, chars: &Vec<char>) -> usize {
    for i in 0..chars.len() {
        let h = &chars[i..lookahead + i];
        let hash = HashSet::<&char>::from_iter(h);
        if hash.len() == lookahead {
            return i + lookahead;
        }
    }
    panic!("Did not find marker!");
}
