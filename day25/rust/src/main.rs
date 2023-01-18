use std::{fs::File, io::BufRead, io::BufReader};

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

fn main() {
    let digits = read_file("input");

    let sum: String = digits
        .into_iter()
        .reduce(add)
        .unwrap()
        .into_iter()
        .map(to_snafu_digit)
        .collect();

    println!("{}", sum);
}

fn add(n1: Vec<i8>, n2: Vec<i8>) -> Vec<i8> {
    let mut carry = 0;
    let mut new_number = Vec::new();
    for val in n1.iter().rev().zip_longest(n2.iter().rev()) {
        let (l, r) = match val {
            Both(l, r) => (l, r),
            Left(v) | Right(v) => (v, &0),
        };

        let n = calculate(*l, *r, &mut carry);
        new_number.push(n);
    }

    if carry != 0 {
        new_number.push(carry);
    }

    new_number.reverse();
    new_number
}

fn calculate(left: i8, right: i8, carry: &mut i8) -> i8 {
    let sum = left + right + *carry;
    if sum > 2 {
        *carry = 1;
    } else if sum < -2 {
        *carry = -1;
    } else {
        *carry = 0;
    }
    (sum + 2).rem_euclid(5) - 2
}

fn from_snafu_digit(c: char) -> i8 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("invalid snafu digit"),
    }
}

fn to_snafu_digit(i: i8) -> char {
    match i {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!("invalid snafu digit"),
    }
}

fn read_file(path: &str) -> Vec<Vec<i8>> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap().chars().map(from_snafu_digit).collect())
        .collect()
}
