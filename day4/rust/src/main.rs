use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn overlaps(&self, range: &Range) -> bool {
        self.from >= range.from && self.from <= range.to
            || self.to >= range.from && self.to <= range.to
    }
    fn contains(&self, range: &Range) -> bool {
        self.from <= range.from && self.to >= range.to
    }
}

fn main() {
    if let Ok(lines) = read_lines("input") {
        let results: Vec<(Range, Range)> = lines.map(parse_line).collect();
        let part1 = results.iter().filter(is_full_overlap).count();
        let part2 = results.iter().filter(is_partial_overlap).count();
        println!("{}", part1);
        println!("{}", part2);
    }
}

fn is_partial_overlap((r1, r2): &&(Range, Range)) -> bool {
    return r1.overlaps(r2) || r2.overlaps(r1);
}
fn is_full_overlap((r1, r2): &&(Range, Range)) -> bool {
    return r1.contains(r2) || r2.contains(r1);
}

fn parse_line(result: Result<String, io::Error>) -> (Range, Range) {
    let line = result.unwrap();
    let parts: Vec<&str> = line.split(",").collect();
    let left: Vec<&str> = parts[0].split("-").collect();
    let right: Vec<&str> = parts[1].split("-").collect();
    (
        Range {
            from: left[0].parse().unwrap(),
            to: left[1].parse().unwrap(),
        },
        Range {
            from: right[0].parse().unwrap(),
            to: right[1].parse().unwrap(),
        },
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
