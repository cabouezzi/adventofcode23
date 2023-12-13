use core::panic;
use std::{cmp, fs};

static mut USE_PART_2: bool = false;

fn compare(lhs: &String, rhs: &String) -> usize {
    assert!(lhs.len() == rhs.len());
    let mut total = 0;
    for i in 0..lhs.len() {
        if lhs[i..i + 1] != rhs[i..i + 1] {
            total += 1
        }
    }
    return total;
}

fn find_row_reflection(pattern: &Vec<String>) -> Option<usize> {
    for i in 1..pattern.len() {
        let reversed: Vec<String> = pattern[i..pattern.len()]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let half: Vec<String> = pattern[0..i].iter().map(|e| e.to_string()).rev().collect();

        let mut diff = 0;
        for j in 0..cmp::min(half.len(), reversed.len()) {
            diff += compare(&half[j], &reversed[j]);
        }
        if diff == if unsafe { USE_PART_2 } { 1 } else { 0 } {
            return Some(i);
        }
    }
    return None;
}

fn find_column_reflection(pattern: &Vec<String>) -> Option<usize> {
    let mut inverse: Vec<String> = vec!["".to_string(); pattern[0].len()];
    for i in 0..pattern[0].len() {
        for j in 0..pattern.len() {
            inverse[i] += &pattern[j][i..i + 1];
        }
    }
    return find_row_reflection(&inverse);
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("should have read the file");
    let patterns = content.split("\n\n").collect::<Vec<_>>();

    fn get_total(patterns: &Vec<&str>) -> usize {
        let mut total = 0;
        for pattern in patterns {
            let lines: Vec<String> = pattern.split("\n").map(|e| e.to_string()).collect();
            if let Some(row_split) = find_row_reflection(&lines) {
                total += 100 * row_split;
            } else if let Some(column_split) = find_column_reflection(&lines) {
                total += column_split;
            } else {
                panic!("Found no reflection for pattern: \n\n{}\n\n", pattern);
            }
        }
        return total;
    }
    unsafe { USE_PART_2 = false };
    println!("Part 1: {}", get_total(&patterns));
    unsafe { USE_PART_2 = true };
    println!("Part 2: {}", get_total(&patterns));
}
