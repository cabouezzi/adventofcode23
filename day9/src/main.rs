use std::fs;

fn predict(nums: &Vec<isize>, part_two: bool) -> isize {
    if nums.iter().all(|&e| e == 0) || nums.is_empty() {
        return 0;
    }
    let differences: Vec<isize> = nums[1..].iter().enumerate().map(|(i, e)| *e - nums[i]).collect();
    if !part_two {
        return nums[nums.len() - 1] + predict(&differences, part_two);
    } else {
        return nums[0] - predict(&differences, part_two);
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("should have read the file");
    let binding = content.replace(".", " ");
    let lines = binding.split("\n").collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;
    for line in lines {
        let ints: Vec<isize> = line
            .split(" ")
            .map(|e| e.parse::<isize>().unwrap())
            .collect();
        p1 += predict(&ints, false);
        p2 += predict(&ints, true);
    }
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
