use std::env::{self};
use std::{collections::HashMap, fs};

use ::num::Integer;

#[derive(Clone)]
struct Instruction {
    left: String,
    right: String,
}

fn main() {
    let mut part2 = false;
    let args: Vec<String> = env::args().collect();
    if let Some(flag) = args.get(1) {
        if flag == "--part-two" {
            part2 = true;
        }
    }
    let content = fs::read_to_string("input.txt").expect("should have read the file");
    let binding = content.replace(".", " ");
    let lines = binding.split("\n").collect::<Vec<_>>();

    let instructions = lines.first().unwrap().to_string();
    let n = lines.len() - 2;

    let mut string2index: HashMap<String, usize> = HashMap::new();
    let mut adjacency = vec![
        Instruction {
            left: "".to_string(),
            right: "".to_string()
        };
        n
    ];

    let network = &lines[2..];
    let mut paths: Vec<usize> = Vec::new();
    for i in 0..network.len() {
        let data = network[i];
        let node = data.to_string()[0..3].to_string();
        if (node[2..3] == "A".to_string() && part2) || (node == "AAA" && !part2) {
            paths.push(i);
        }
        string2index.insert(node, i);
        adjacency[i].left = data.to_string()[7..10].to_string();
        adjacency[i].right = data.to_string()[12..15].to_string();
    }

    println!(
        "Found {} paths, {} instructions each",
        paths.len(),
        instructions.len()
    );

    let mut req_steps = vec![0; paths.len()];
    // While not all paths are on Z
    for i in 0..paths.len() {
        let mut steps: usize = 0;
        let mut cur = paths[i];
        while network[cur][2..3] != "Z".to_string() {
            for c in instructions.chars() {
                cur = match c {
                    'L' => *string2index.get(&adjacency[cur].left).unwrap(),
                    'R' => *string2index.get(&adjacency[cur].right).unwrap(),
                    _ => panic!("Unknown instruction {c}"),
                };
            }
            steps += instructions.len();
        }
        req_steps[i] = steps;
    }

    let total = req_steps.iter().fold(req_steps[0], |acc, e| acc.lcm(e));
    println!("Steps: {}", total);
}
