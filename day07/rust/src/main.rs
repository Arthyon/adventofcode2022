use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
struct NodeId {
    name: String,
    index: usize,
}

#[derive(Clone, Debug)]
enum FileItem {
    File(i32),
    Folder,
}
#[derive(Clone)]
struct Node {
    id: NodeId,
    parent: Option<usize>,
    data: FileItem,
    children: Vec<NodeId>,
}

fn get_child_node<'a>(node: &Node, line: String) -> usize {
    let dir_name = &line[5..];
    for n in &node.children {
        if n.name == dir_name {
            return n.index;
        }
    }
    print!("{}", dir_name);
    panic!("No such node");
}
fn get_dir_name(line: String) -> String {
    return String::from(&line[4..]);
}
fn get_file_info(line: String) -> (String, i32) {
    let parts = line.split(" ").collect::<Vec<&str>>();
    return (String::from(parts[1]), parts[0].parse::<i32>().unwrap());
}

fn handle_cd<'a>(line: String, nodes: &'a Vec<Node>, current_index: usize) -> usize {
    let dir: String = line.chars().skip(5).collect();
    match dir.as_str() {
        "/" => return nodes[0].id.index,
        ".." => {
            let i = nodes[current_index].parent.unwrap();
            return nodes[i].id.index;
        }
        _ => nodes[get_child_node(&nodes[current_index], line)].id.index,
    }
}

fn calculate_folder_size(all_nodes: &Vec<Node>, node: &Node, current: i32) -> i32 {
    if let FileItem::File(size) = node.data {
        current + size
    } else {
        let mut c = 0;
        for n in &node.children {
            let val = calculate_folder_size(all_nodes, &all_nodes[n.index], 0);
            c = c + val;
        }
        current + c
    }
}

fn calculate_folder_sizes(all_nodes: &Vec<Node>, node: &Node) -> Option<i32> {
    if let FileItem::Folder = node.data {
        let size = calculate_folder_size(all_nodes, node, 0);
        Some(size)
    } else {
        None
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut nodes = Vec::<Node>::new();
        nodes.push(Node {
            id: NodeId {
                name: String::from("/"),
                index: 0,
            },
            parent: None,
            data: FileItem::Folder,
            children: Vec::new(),
        });
        let mut current_index = 0;

        for result in lines {
            if let Ok(line) = result {
                match &line[..4] {
                    "$ cd" => {
                        current_index = handle_cd(line, &nodes, current_index);
                    }
                    "$ ls" => (),
                    "dir " => {
                        let name = get_dir_name(line);
                        let next_index = nodes.len();
                        let node = Node {
                            id: NodeId {
                                name: name.clone(),
                                index: next_index,
                            },
                            parent: Some(current_index),
                            children: Vec::new(),
                            data: FileItem::Folder,
                        };
                        nodes.push(node);
                        nodes[current_index].children.push(NodeId {
                            name: name.clone(),
                            index: next_index,
                        });
                    }
                    _ => {
                        let (name, size) = get_file_info(line);
                        let next_index = nodes.len();
                        let node = Node {
                            id: NodeId {
                                name: name.clone(),
                                index: next_index,
                            },
                            parent: Some(current_index),
                            children: Vec::new(),
                            data: FileItem::File(size),
                        };
                        nodes.push(node);
                        nodes[current_index].children.push(NodeId {
                            name: name.clone(),
                            index: next_index,
                        });
                    }
                }
            }
        }
        let sizes: Vec<i32> = nodes
            .iter()
            .filter_map(|n| calculate_folder_sizes(&nodes, n))
            .collect();
        let part1 = sizes.iter().filter(|s| **s <= 100000).sum::<i32>();
        let free_space = 70000000 - sizes.iter().max().unwrap();
        let needed_space = 30000000 - free_space;
        let part2 = sizes.iter().filter(|s| **s >= needed_space).min().unwrap();
        println!("{}", part1);
        println!("{}", part2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
