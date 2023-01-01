use std::{
    collections::HashMap,
    fs::{self},
};

fn main() {
    // Strip last newline from input first
    let s = fs::read_to_string("input").unwrap();
    let parts: Vec<&str> = s.split("\n\n").collect();
    let actions = get_actions(parts[1]);
    let stacks = get_stacks(parts[0]);
    let part1 = move_stacks(&stacks, &actions, crate_mover_9000);
    let part2 = move_stacks(&stacks, &actions, crate_mover_9001);

    print_stacks(part1);
    print_stacks(part2);
}

fn move_stacks(
    stacks: &HashMap<u32, Vec<char>>,
    actions: &Vec<(u32, u32, u32)>,
    mover: fn(&mut Vec<char>, &mut Vec<char>),
) -> HashMap<u32, Vec<char>> {
    let mut stacks = stacks.clone();
    for (amount, from, to) in actions {
        let stack = stacks.get_mut(from).unwrap();
        let mut temp = Vec::new();

        for _ in 0..*amount {
            let val = stack.pop().unwrap();
            temp.push(val);
        }
        let stack = stacks.get_mut(to).unwrap();
        mover(stack, &mut temp);
        // temp.reverse();
        // stack.append(&mut temp);
    }

    stacks
}

fn print_stacks(stacks: HashMap<u32, Vec<char>>) {
    let mut r = Vec::from_iter(stacks.iter()); //.sort_by_key(|(k, v)| k);
    r.sort_by_key(|(k, _)| **k);

    for (_, s) in r {
        print!("{}", s.last().unwrap());
    }
    println!("");
}

fn crate_mover_9000(stack: &mut Vec<char>, items: &mut Vec<char>) {
    stack.append(items);
}

fn crate_mover_9001(stack: &mut Vec<char>, items: &mut Vec<char>) {
    items.reverse();
    stack.append(items);
}

fn get_stacks(data: &str) -> HashMap<u32, Vec<char>> {
    let mut map = HashMap::new();
    let mut rows = data
        .split("\n")
        .map(|f| f.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let numbers = rows.pop().unwrap();

    for x in (1..rows[0].len()).step_by(4) {
        let s = numbers[x].to_digit(10).unwrap();
        let mut v = Vec::new();
        for r in rows.iter().rev() {
            if r[x] != ' ' {
                v.push(r[x]);
            }
        }
        map.insert(s, v);
    }
    map
}

fn get_actions(actions: &str) -> Vec<(u32, u32, u32)> {
    actions
        .split("\n")
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (
                parts[1].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[5].parse().unwrap(),
            )
        })
        .collect()
}
