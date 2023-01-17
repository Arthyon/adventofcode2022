use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;
use s5::{
    constants::Const, equations::Equation, formulas::Formula, solver_state::SolverState,
    variables::Variable,
};

enum Yell {
    Number(i64),
    Operation(String, String, String),
}

enum Either {
    Equation(Equation),
    VariableBinding(Variable, Const),
}

const HUMAN: &str = "humn";
const ROOT: &str = "root";

fn main() {
    let mut monkeys: HashMap<_, _> = read_file("input").map(parse_line).collect();

    part_1(&monkeys);
    part_2(&mut monkeys);
}

fn part_1(monkeys: &HashMap<String, Yell>) {
    let mut solver = build_solver_state(monkeys, Vec::new());
    find(&mut solver, ROOT);
}

fn part_2(monkeys: &mut HashMap<String, Yell>) {
    monkeys.remove(HUMAN);
    if let Some(Yell::Operation(l, _, r)) = monkeys.remove(ROOT) {
        let l = Formula::from(Variable::new(l.as_str(), ""));
        let r = Formula::from(Variable::new(r.as_str(), ""));

        let equations = vec![Equation::new(l, r)];
        let mut solver = build_solver_state(monkeys, equations);
        find(&mut solver, HUMAN);
    } else {
        panic!("root not found");
    }
}

fn find(solver: &mut SolverState, var: &str) {
    let find = Variable::new(var, "");
    solver.deduce();
    let s = solver.get_binding(&find).unwrap();
    println!("{}", s);
}

fn build_solver_state(
    monkeys: &HashMap<String, Yell>,
    mut equations: Vec<Equation>,
) -> SolverState {
    let mut solver = SolverState::new();
    let mut variables = Vec::new();

    for (s, y) in monkeys {
        match parse_monkey_yell(s, y) {
            Either::VariableBinding(name, value) => variables.push((name, value)),
            Either::Equation(equation) => equations.push(equation),
        }
    }

    solver.add_equations(&equations);

    for (s, i) in variables {
        solver.bind_variable(&s, &i);
    }

    solver
}

fn parse_monkey_yell(s: &String, yell: &Yell) -> Either {
    let s = Variable::new(s.as_str(), "");
    match yell {
        Yell::Number(i) => Either::VariableBinding(s, Const::from(*i)),
        Yell::Operation(l, o, r) => {
            let l = Variable::new(l.as_str(), "");
            let r = Variable::new(r.as_str(), "");
            let f = get_formula(l, o, r);
            Either::Equation(Equation::new_assignment(&s, f))
        }
    }
}

fn get_formula(left: Variable, o: &String, right: Variable) -> Formula {
    match o.as_str() {
        "+" => left + right,
        "-" => left - right,
        "/" => left / right,
        "*" => left * right,
        _ => panic!("Invalid operator"),
    }
}

fn parse_line(line: String) -> (String, Yell) {
    let parts: Vec<_> = line.split(":").map(|f| f.trim()).collect();
    let r = Regex::new(r"(.*) ([+-/*]) (.*)").unwrap();

    match r.captures(&parts[1]) {
        None => (
            parts[0].to_string(),
            Yell::Number(parts[1].parse().unwrap()),
        ),
        Some(c) => (
            parts[0].to_string(),
            Yell::Operation(
                c.get(1).unwrap().as_str().to_string(),
                c.get(2).unwrap().as_str().to_string(),
                c.get(3).unwrap().as_str().to_string(),
            ),
        ),
    }
}

fn read_file(path: &str) -> impl Iterator<Item = String> {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().map(|f| f.unwrap())
}
