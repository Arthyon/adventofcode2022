use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = "input";

    let list = read_file(file, 1);
    let mixed_list = mix(list, 1);
    decode(mixed_list);

    let list = read_file(file, 811589153);
    let mixed_list = mix(list, 10);
    decode(mixed_list);
}

fn create_list(input: Vec<(usize, i64)>, indices: Vec<usize>) -> Vec<i64> {
    indices.iter().map(|idx| input[*idx].1).collect()
}

fn mix(input: Vec<(usize, i64)>, times: usize) -> Vec<i64> {
    let mut indices: Vec<_> = input.clone().into_iter().map(|(i, _)| i).collect();
    let length = i64::try_from(indices.len()).unwrap() - 1;

    for _ in 0..times {
        for (idx, val) in &input {
            let current_index = indices.iter().position(|i| i == idx).unwrap();
            indices.remove(current_index);
            let new_index = (i64::try_from(current_index).unwrap() + val).rem_euclid(length);
            indices.insert(usize::try_from(new_index).unwrap(), *idx);
        }
    }

    create_list(input, indices)
}

fn decode(list: Vec<i64>) {
    let start_index = list.iter().position(|v| *v == 0).unwrap();
    let length = list.len();
    let sum: i64 = [1000, 2000, 3000]
        .iter()
        .map(|n| (start_index + n).rem_euclid(length))
        .map(|idx| list[idx])
        .sum();
    println!("{}", sum);
}

fn read_file(path: &str, decryption_key: i64) -> Vec<(usize, i64)> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
        .lines()
        .map(|f| f.unwrap().parse::<i64>().unwrap() * decryption_key)
        .enumerate()
        .collect()
}
