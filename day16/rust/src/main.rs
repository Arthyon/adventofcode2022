use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use michie::memoized;

struct Room {
    flow: i32,
    connections: Vec<String>,
}

fn main() {
    let rooms: HashMap<_, _> = read_file("input").map(parse_line).collect();
    let interesting_valves: Vec<_> = rooms
        .iter()
        .filter(|(_, room)| room.flow > 0)
        .map(|(k, _)| k.clone())
        .collect();

    let max_pressure_part1 = calculate_max_pressure(
        "AA".to_string(),
        30,
        interesting_valves.clone(),
        false,
        &rooms,
    );

    println!("{}", max_pressure_part1);

    let max_pressure_part2 = calculate_max_pressure(
        "AA".to_string(),
        26,
        interesting_valves.clone(),
        true,
        &rooms,
    );

    println!("{}", max_pressure_part2);
}

#[memoized(key_expr = to_key(&location, minutes, &interesting_valves, elephant), store_type = HashMap<String, i32>)]
fn calculate_max_pressure(
    location: String,
    minutes: i32,
    interesting_valves: Vec<String>,
    elephant: bool,
    all_valves: &HashMap<String, Room>,
) -> i32 {
    let base_score = all_valves.get(&location).unwrap().flow * minutes;
    let mut highest_score = 0;
    let valve_copy = interesting_valves.clone();
    for valve in &interesting_valves {
        let path_length = shortest_path(all_valves, &location, &valve) + 1;
        if path_length <= minutes {
            let new = valve_copy
                .iter()
                .filter(|v| *v != valve)
                .map(|v| v.clone())
                .collect();

            let max_pressure = calculate_max_pressure(
                valve.clone(),
                minutes - path_length,
                new,
                elephant,
                all_valves,
            );
            highest_score = std::cmp::max(highest_score, max_pressure);
        }
    }
    if elephant {
        let human_max = calculate_max_pressure(
            "AA".to_string(),
            26,
            interesting_valves,
            !elephant,
            all_valves,
        );
        highest_score = std::cmp::max(highest_score, human_max);
    }
    base_score + highest_score
}

fn to_key(
    location: &String,
    minutes: i32,
    interesting_valves: &Vec<String>,
    elephant: bool,
) -> String {
    let mut valves = interesting_valves
        .iter()
        .map(|s| s.clone())
        .collect::<Vec<_>>();
    valves.sort_unstable();
    format!("{}{}{}{}", location, minutes, valves.join(""), elephant)
}

#[memoized(key_expr = (from.clone(),to.clone()), store_type = HashMap<(String, String), i32>)]
fn shortest_path(connections: &HashMap<String, Room>, from: &String, to: &String) -> i32 {
    let mut visited = HashSet::from([from.clone()]);
    let mut queue = VecDeque::from([(from.clone(), 0)]);

    let min_distance;
    loop {
        match queue.pop_front() {
            None => panic!("No available path!"),
            Some((location, distance)) => {
                if &location == to {
                    min_distance = distance;
                    break;
                }
                let neighbours: Vec<_> = connections
                    .get(&location)
                    .unwrap()
                    .connections
                    .iter()
                    .filter(|v| !visited.contains(&**v))
                    .collect();

                for n in neighbours {
                    visited.insert(n.clone());
                    queue.push_back((n.clone(), distance + 1));
                }
            }
        }
    }

    min_distance
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().map(|l| l.unwrap())
}
fn parse_line(line: String) -> (String, Room) {
    let r =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z\s,]*)")
            .unwrap();

    let captures = r.captures(&line).unwrap();
    let name = captures.get(1).unwrap().as_str().to_string();
    let flow = captures.get(2).unwrap().as_str().parse().unwrap();
    let tunnels = captures
        .get(3)
        .unwrap()
        .as_str()
        .split(",")
        .map(|v| v.trim().to_string())
        .collect();
    (
        name,
        Room {
            flow: flow,
            connections: tunnels,
        },
    )
}
