use std::fs;
use regex::Regex;

fn check_line(springs: String, arrangements: Vec<isize>) -> bool {
    let regex = Regex::new(r"#+.*?").unwrap();
    let groups: Vec<&str> = regex.find_iter(&springs).map(|m| m.as_str()).collect();
    println!("{:?} {:?}", springs, arrangements);
    println!("{:?}", groups);
    return groups.iter().enumerate().all(|(i, e)| *arrangements.get(i).unwrap_or(&-1) == e.len() as isize);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("should have read the file");
    let lines = input.split("\n").collect::<Vec<_>>();
    let springs = lines.iter().map(|line| line.split_once(" ").unwrap().0);
    let arrangements = lines.iter().map(|line| line.split_once(" ").unwrap().1);

    println!("{}", check_line("#.#.###".to_string(), [1, 1, 3].to_vec()));

    println!("Hello, world!");
}
