use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = lines_from_file("input");
    part_1(&lines);
    part_2(&lines);
}

fn part_1(lines: &Vec<String>) {
    let mut sum = 0;
    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);
        let mut v = vec![
            HashSet::from_iter(left.chars()),
            HashSet::from_iter(right.chars()),
        ];
        sum += intersect(&mut v).iter().map(get_priority).sum::<u32>();
    }
    println!("{}", sum);
}

fn part_2(lines: &Vec<String>) {
    let mut sum = 0;
    for chunk in lines.chunks(3) {
        let mut d: Vec<HashSet<char>> = chunk
            .iter()
            .map(|f| HashSet::from_iter(f.chars()))
            .collect();
        sum += intersect(&mut d).iter().map(get_priority).sum::<u32>();
    }
    println!("{}", sum);
}

fn intersect(vecs: &mut Vec<HashSet<char>>) -> Vec<char> {
    let (intersection, others) = vecs.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e));
    }
    return intersection.iter().map(|c| c.clone()).collect();
}

fn get_priority(c: &char) -> u32 {
    let code = c.clone() as u32;
    if c.is_lowercase() {
        code - 96
    } else {
        code - 38
    }
}

fn lines_from_file(path: &str) -> Vec<String> {
    let input = File::open(path).expect("Failed to read file");
    io::BufReader::new(input)
        .lines()
        .map(|c| c.expect("Could not parse line"))
        .collect()
}
