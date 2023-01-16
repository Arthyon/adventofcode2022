use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use michie::memoized;
use regex::Regex;

struct Robot {
    robot_type: usize,
    cost: [i32; 4],
}

struct Blueprint {
    number: i32,
    ore_robot: Robot,
    clay_robot: Robot,
    obsidian_robot: Robot,
    geode_robot: Robot,
    cost_matrix: [[i32; 4]; 4],
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    resources: [i32; 4],
    robots: [i32; 4],
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

fn main() {
    let blueprints: Vec<_> = read_file("input").map(parse_line).collect();
    let initial_state = State {
        resources: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    };

    let part_1 = part_1(&blueprints, &initial_state);

    println!("{}", part_1);

    let part_2 = part_2(&blueprints.into_iter().take(3).collect(), &initial_state);

    println!("{}", part_2);
}

fn part_1(blueprints: &Vec<Blueprint>, state: &State) -> i32 {
    let mut score = 0;
    for blueprint in blueprints {
        let geodes = evaluate_blueprint(&blueprint, state.clone(), 24);
        score += blueprint.number * geodes;
    }

    score
}

fn part_2(blueprints: &Vec<Blueprint>, state: &State) -> i32 {
    let mut total_geodes = 1;
    for blueprint in blueprints {
        let geodes = evaluate_blueprint(&blueprint, state.clone(), 32);
        total_geodes *= geodes;
    }

    total_geodes
}

fn evaluate_blueprint(blueprint: &Blueprint, state: State, minutes: i32) -> i32 {
    if minutes == 0 {
        state.resources[GEODE]
    } else {
        let mut max = 0;
        for state in determine_next_states(blueprint, &state, minutes) {
            let score = evaluate_branch(blueprint, state, minutes);
            max = std::cmp::max(max, score);
        }
        max
    }
}

fn determine_next_states(blueprint: &Blueprint, state: &State, minutes: i32) -> Vec<State> {
    if minutes == 1 {
        vec![increment_resources(&state)]
    } else {
        if can_afford(&blueprint.geode_robot, state) {
            vec![buy_robot(&blueprint.geode_robot, state)]
        } else {
            let mut v = Vec::from([increment_resources(state)]);
            if need_more_robots(blueprint, state, OBSIDIAN)
                && can_afford(&blueprint.obsidian_robot, state)
            {
                v.push(buy_robot(&blueprint.obsidian_robot, state));
            } else {
                if need_more_robots(blueprint, state, CLAY)
                    && can_afford(&blueprint.clay_robot, state)
                {
                    v.push(buy_robot(&blueprint.clay_robot, state));
                }
                if need_more_robots(blueprint, state, ORE)
                    && can_afford(&blueprint.ore_robot, state)
                {
                    v.push(buy_robot(&blueprint.ore_robot, state));
                }
            }
            v
        }
    }
}

fn increment_resources(state: &State) -> State {
    State {
        resources: map_array(state.resources, |(i, r)| r + state.robots[i]),
        ..*state
    }
}

fn can_afford(robot: &Robot, state: &State) -> bool {
    state
        .resources
        .iter()
        .zip(&robot.cost)
        .all(|(resource, cost)| resource >= cost)
}

fn need_more_robots(blueprint: &Blueprint, state: &State, resource_type: usize) -> bool {
    let max_cost = blueprint.cost_matrix[resource_type].iter().max().unwrap();
    state.robots[resource_type] < *max_cost
}

fn buy_robot(robot: &Robot, state: &State) -> State {
    let state = increment_resources(state);
    let resources = state
        .resources
        .iter()
        .zip(&robot.cost)
        .map(|(resource, cost)| resource - cost);
    let mut new_robots = state.robots.clone();
    new_robots[robot.robot_type] += 1;

    State {
        resources: to_array(resources),
        robots: new_robots,
    }
}

fn map_array(arr: [i32; 4], mapper: impl Fn((usize, &i32)) -> i32) -> [i32; 4] {
    to_array(arr.iter().enumerate().map(mapper))
}

fn to_array(it: impl Iterator<Item = i32>) -> [i32; 4] {
    it.collect::<Vec<_>>().try_into().unwrap()
}

#[memoized(key_expr = (minutes,blueprint.number, state.clone()), store_type = HashMap<(i32,i32,State), i32>)]
fn evaluate_branch(blueprint: &Blueprint, state: State, minutes: i32) -> i32 {
    evaluate_blueprint(blueprint, state, minutes - 1)
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().map(|f| f.unwrap())
}

fn parse_line(line: String) -> Blueprint {
    let r = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let captures = r.captures(&line).unwrap();
    let int = |i| captures.get(i).unwrap().as_str().parse::<i32>().unwrap();
    Blueprint {
        number: int(1),
        ore_robot: Robot {
            robot_type: ORE,
            cost: [int(2), 0, 0, 0],
        },
        clay_robot: Robot {
            robot_type: CLAY,
            cost: [int(3), 0, 0, 0],
        },
        obsidian_robot: Robot {
            robot_type: OBSIDIAN,
            cost: [int(4), int(5), 0, 0],
        },
        geode_robot: Robot {
            robot_type: GEODE,
            cost: [int(6), 0, int(7), 0],
        },
        cost_matrix: [
            [int(2), int(3), int(4), int(6)],
            [0, 0, int(5), 0],
            [0, 0, 0, int(7)],
            [0, 0, 0, 0],
        ],
    }
}
