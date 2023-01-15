use std::{
    collections::{HashMap, HashSet},
    fs::{self},
};

struct Rock {
    shape: Vec<(i128, i128)>,
}

fn main() {
    let movement = get_movement("input");
    let rocks = create_rocks();
    let part1_height = rockfall(2022, &movement, &rocks);
    println!("{}", part1_height);

    let part2_height = rockfall(1000000000000, &movement, &rocks);
    println!("{}", part2_height);
}

fn rockfall(iterations: i128, movement: &Vec<char>, rocks: &Vec<Rock>) -> i128 {
    let mut i = 0;
    let mut repeats: HashMap<(usize, usize), (i32, i128, i128)> = HashMap::new();
    let mut tower = HashSet::new();

    let rock_count = i128::try_from(rocks.len()).unwrap();
    let mut wind_idx = movement.len() - 1;
    let mut added_height_by_repeating = 0;

    while i < iterations {
        let next_rock = usize::try_from(i % rock_count).unwrap();
        let rock = &rocks[next_rock];
        let (fallen_rock, new_wind_idx) = release_rock(wind_idx, &movement, &rock, &tower);
        wind_idx = new_wind_idx;
        tower.extend(fallen_rock);

        if added_height_by_repeating == 0 {
            let (added_height, rocks) = check_repetitions(
                next_rock,
                wind_idx,
                &mut repeats,
                get_height(&tower),
                &i,
                &iterations,
            );

            added_height_by_repeating = added_height;
            i = rocks;
        }
        i += 1;
    }

    get_height(&tower) + added_height_by_repeating
}

fn get_height(tower: &HashSet<(i128, i128)>) -> i128 {
    if tower.len() == 0 {
        0
    } else {
        let (_, max_y) = tower.iter().max_by_key(|(_, y)| y).unwrap();
        *max_y + 1
    }
}

fn check_repetitions(
    current_shape: usize,
    current_wind: usize,
    repeats: &mut HashMap<(usize, usize), (i32, i128, i128)>,
    current_height: i128,
    current_rocks: &i128,
    target: &i128,
) -> (i128, i128) {
    let key = (current_shape, current_wind);
    if repeats.contains_key(&key) {
        let (repetitions, rocks, height) = repeats.get(&key).unwrap();
        if *repetitions == 3 {
            let delta_height = current_height - height;
            let delta_rocks = current_rocks - rocks;
            let repeats = (target - current_rocks) / delta_rocks;
            return (
                repeats * delta_height,
                current_rocks + (repeats * delta_rocks),
            );
        }
        repeats.insert(key, (repetitions + 1, *current_rocks, current_height));
    } else {
        repeats.insert(key, (1, *current_rocks, current_height));
    }

    (0, *current_rocks)
}

fn release_rock(
    mut wind_idx: usize,
    movement: &Vec<char>,
    rock: &Rock,
    tower: &HashSet<(i128, i128)>,
) -> (Vec<(i128, i128)>, usize) {
    let height = get_height(tower);
    let mut spawned_rock = spawn_rock(height, rock);
    loop {
        wind_idx = (wind_idx + 1) % movement.len();

        blow_wind(&mut spawned_rock, movement[wind_idx], &tower);
        let fallen_rock = spawned_rock.iter().map(|(x, y)| (*x, y - 1)).collect();
        if can_move_rock(&fallen_rock, &tower) {
            spawned_rock = fallen_rock;
        } else {
            return (spawned_rock, wind_idx);
        }
    }
}

fn blow_wind(rock: &mut Vec<(i128, i128)>, direction: char, tower: &HashSet<(i128, i128)>) {
    let blown_rock: Vec<(i128, i128)> = rock
        .iter()
        .map(|(x, y)| (if direction == '>' { x + 1 } else { x - 1 }, *y))
        .collect();

    if can_move_rock(&blown_rock, tower) {
        *rock = blown_rock;
    }
}

fn can_move_rock(rock: &Vec<(i128, i128)>, tower: &HashSet<(i128, i128)>) -> bool {
    fn out_of_bounds(x: &i128, y: &i128) -> bool {
        x < &0 || x >= &7 || y < &0
    }
    let collides = rock
        .iter()
        .any(|(x, y)| out_of_bounds(x, y) || tower.contains(&(*x, *y)));
    !collides
}

fn spawn_rock(height: i128, rock: &Rock) -> Vec<(i128, i128)> {
    rock.shape
        .iter()
        // TODO Check if spawn is correct
        .map(|(x, y)| (x + 2, y + height + 3))
        .collect()
}

fn create_rocks() -> Vec<Rock> {
    vec![
        Rock {
            shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        }, // -
        Rock {
            shape: vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        }, // +
        Rock {
            shape: vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
        }, // J
        Rock {
            shape: vec![(0, 3), (0, 2), (0, 1), (0, 0)],
        }, // I
        Rock {
            shape: vec![(0, 1), (1, 1), (0, 0), (1, 0)],
        }, // .
    ]
}

fn get_movement(path: &str) -> Vec<char> {
    fs::read_to_string(path).unwrap().chars().collect()
}
