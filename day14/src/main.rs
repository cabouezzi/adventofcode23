use std::fs;

fn rotate_left(platform: &Vec<String>) -> Vec<String> {
    let mut rot: Vec<String> = vec!["".to_string(); platform[0].len()];
    for col in 0..platform[0].len() {
        for row in 0..platform.len() {
            rot[platform[0].len() - col - 1]
                .push(platform[row].chars().collect::<Vec<char>>()[col]);
        }
    }
    return rot;
}

fn tilt_line(line: &String) -> String {
    let mut chars = line.chars().collect::<Vec<char>>();
    let mut open = 0;
    for i in 0..chars.len() {
        if chars[open] != '.' {
            open += 1;
            continue;
        }
        match chars[i] {
            '.' => {}
            'O' => {
                chars[open] = 'O';
                open += 1;
                chars[i] = '.';
            }
            '#' => {
                open = i + 1;
            }
            _ => {}
        }
    }
    return chars.iter().collect();
}

fn left_weight(line: &String) -> usize {
    let mut total = 0;
    for (index, character) in line.chars().enumerate() {
        if character == 'O' {
            total += line.len() - index;
        }
    }
    return total;
}

fn tilt_platform(platform: &Vec<String>) -> Vec<String> {
    return platform.iter().map(|e| tilt_line(e)).collect();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let mut platform = input
        .lines()
        // Reverse the strings cuz I don't feel like modifying for part 2
        .map(|e| e.to_string().chars().rev().collect())
        .collect::<Vec<String>>();
    platform = rotate_left(&platform); // Cuz left is north

    fn compute_weight(platform: &Vec<String>) -> usize {
        let mut total = 0;
        for line in platform {
            total += left_weight(&line);
        }
        return total;
    }

    println!("Part 1: {}", compute_weight(&tilt_platform(&platform)));

    let target = 1000000000;
    let mut states: Vec<Vec<String>> = Vec::new();
    for i in 0..target {
        // Run cycle
        for _ in 0..4 {
            platform = tilt_platform(&platform);
            platform = rotate_left(&platform);
        }
        println!("Update {}: {}", i, compute_weight(&platform));
        if let Some(last) = states.iter().position(|e| *e == platform) {
            // Cycle consists of items between cur and last
            let cycle: Vec<Vec<String>> = states[last..].iter().map(|e| e.clone()).collect();
            let leftover = target - i - 1;
            println!(
                "Part 2: {} {}",
                last,
                compute_weight(&cycle[leftover % cycle.len()])
            );
            break;
        } else {
            states.push(platform.clone());
        }
    }
}
