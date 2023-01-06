use regex::{Captures, Match, Regex};
use std::fs;

struct Monkey {
    items: Vec<i64>,
    divisor: i64,
    operation: Box<dyn Fn(i64) -> i64>,
    true_monkey: usize,
    false_monkey: usize,
    items_inspected: usize,
}

fn main() {
    let path = "input";
    part_1(path);
    part_2(path);
}

fn part_1(path: &str) {
    let mut monkeys = parse_input(path);
    perform_rounds(&mut monkeys, |i| i / 3, 20);
}

fn part_2(path: &str) {
    let mut monkeys = parse_input(path);
    let division_constant: i64 = monkeys.iter().map(|m| m.divisor).product();
    perform_rounds(&mut monkeys, |i| i % division_constant, 10000);
}

fn parse_input(path: &str) -> Vec<Monkey> {
    read_file(path).into_iter().map(map_monkey).collect()
}

fn perform_rounds<T>(monkeys: &mut Vec<Monkey>, worry_reducer: T, rounds: i32)
where
    T: Fn(i64) -> i64,
{
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];

            let items: Vec<_> = monkey
                .items
                .drain(..)
                .map(|item| (monkey.operation)(item))
                .map(&worry_reducer)
                .collect();
            monkey.items_inspected += items.len();

            let (divisor, true_monkey, false_monkey) =
                (monkey.divisor, monkey.true_monkey, monkey.false_monkey);

            for item in items {
                let new_monkey = if item % divisor == 0 {
                    true_monkey
                } else {
                    false_monkey
                };
                monkeys[new_monkey].items.push(item);
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    let monkey_business: usize = inspections.iter().take(2).product();

    println!("{}", monkey_business);
}

fn match_as_usize(m: Option<Match>) -> usize {
    m.unwrap().as_str().parse().unwrap()
}

fn map_monkey(s: String) -> Monkey {
    let r = Regex::new(
        r"Monkey (\d+):\n\s{2}Starting items: (.*)\n\s{2}Operation: new = (.*) ([+-/\*]) (.*)\n\s{2}Test: divisible by (\d+)\n\s{4}If true: throw to monkey (\d+)\n\s{4}If false: throw to monkey (\d+)",
    )
    .unwrap();
    let captures = r.captures(&s).unwrap();
    Monkey {
        items: captures
            .get(2)
            .unwrap()
            .as_str()
            .split(",")
            .map(|f| f.trim().parse().unwrap())
            .collect(),
        divisor: captures.get(6).unwrap().as_str().parse().unwrap(),
        operation: create_operation(&captures),
        true_monkey: match_as_usize(captures.get(7)),
        false_monkey: match_as_usize(captures.get(8)),
        items_inspected: 0,
    }
}

fn create_operation(captures: &Captures) -> Box<dyn Fn(i64) -> i64> {
    let parts = vec![
        captures.get(3).unwrap().as_str(),
        captures.get(4).unwrap().as_str(),
        captures.get(5).unwrap().as_str(),
    ];
    match parts[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),
        ["old", "*", y] => {
            let y: i64 = y.parse().unwrap();
            Box::new(move |x| x * y)
        }
        ["old", "+", y] => {
            let y: i64 = y.parse().unwrap();
            Box::new(move |x| x + y)
        }
        _ => panic!("{parts:?}"),
    }
}

fn read_file(path: &str) -> Vec<String> {
    let s = fs::read_to_string(path).unwrap();
    s.split("\n\n").map(|f| f.to_string()).collect()
}
