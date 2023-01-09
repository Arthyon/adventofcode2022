use std::{cmp::Ordering, collections::VecDeque, fs};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Element {
    List(Vec<Element>),
    Item(u32),
}

fn main() {
    let f = fs::read_to_string("input").unwrap();
    let elements: Vec<_> = f
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect();

    part_1(&elements);
    part_2(elements);
}

fn part_1(elements: &Vec<Element>) {
    let sum: usize = elements
        .chunks(2)
        .enumerate()
        .filter_map(|(i, el)| {
            if compare(&el[0], &el[1]) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();
    println!("{}", sum);
}

fn part_2(mut elements: Vec<Element>) {
    let div1 = parse_line("[[2]]");
    let div2 = parse_line("[[6]]");
    elements.push(div1.clone());
    elements.push(div2.clone());
    elements.sort_by(compare);

    let product: usize = elements
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if *v == div1 || *v == div2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();
    println!("{}", product);
}

fn compare(left: &Element, right: &Element) -> Ordering {
    match (left, right) {
        (Element::Item(l), Element::Item(r)) => l.cmp(&r),
        (Element::Item(l), Element::List(r)) => compare_lists(&vec![Element::Item(*l)], r),
        (Element::List(l), Element::Item(r)) => compare_lists(l, &vec![Element::Item(*r)]),
        (Element::List(l), Element::List(r)) => compare_lists(l, r),
    }
}

fn compare_lists(l: &Vec<Element>, r: &Vec<Element>) -> Ordering {
    for (left, right) in l.iter().zip(r) {
        match compare(left, right) {
            Ordering::Equal => (),
            ordering => return ordering,
        }
    }

    l.len().cmp(&r.len())
}

fn parse_line(line: &str) -> Element {
    let arr = VecDeque::from_iter(line.chars());
    let (el, _) = parse_array(arr);
    el
}

fn parse_array(mut chars: VecDeque<char>) -> (Element, VecDeque<char>) {
    let mut braces = -1;
    let mut arr = VecDeque::new();
    loop {
        if let Some(c) = chars.pop_front() {
            arr.push_back(c);
            match c {
                '[' => braces += 1,
                ']' if braces > 0 => braces -= 1,
                ']' => {
                    strip_braces(&mut arr);
                    return (parse_array_content(arr), chars);
                }
                _ => {}
            };
        }
    }
}

fn strip_braces(arr: &mut VecDeque<char>) {
    arr.pop_back();
    arr.pop_front();
}

fn to_number(arr: &Vec<char>) -> Option<u32> {
    match arr.iter().collect::<String>().parse() {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

fn parse_array_content(mut array: VecDeque<char>) -> Element {
    let mut elements = VecDeque::new();
    let mut current = Vec::new();

    loop {
        match array.pop_front() {
            Some(',') => {
                match to_number(&current) {
                    Some(v) => elements.push_back(Element::Item(v)),
                    None => (),
                }
                current.clear();
            }
            Some('[') => {
                array.push_front('[');
                let (el, rest) = parse_array(array);
                array = rest;
                elements.push_back(el);
            }
            Some(c) => current.push(c),
            None => {
                match to_number(&current) {
                    Some(v) => elements.push_back(Element::Item(v)),
                    None => (),
                }
                break;
            }
        }
    }
    Element::List(Vec::from(elements))
}
