use std::{collections::HashMap, fs, isize, iter::zip};

fn main() {
    let filepath = "./inputs/input2.txt";
    let contents = fs::read_to_string(filepath).expect("there should of been a file.");

    problem2_part1(&contents);
}

fn decreasing_with_damping(numbers: &Vec<isize>) -> bool {
    let mut bad_levels = 0;
    let mut last_level = numbers[0];
    for i in 1..numbers.len() {
        let diff = last_level - numbers[i];

        if diff < 1 || diff > 3 {
            bad_levels += 1;
        } else {
            last_level = numbers[i];
        }

        if bad_levels > 1 {
            println!("dec: {:?}", &numbers);
            return false;
        }
    }
    return true;
}

fn increasing_with_damping(numbers: &Vec<isize>) -> bool {
    let mut bad_levels = 0;
    let mut last_level = numbers[0];
    for i in 1..numbers.len() {
        let diff = numbers[i] - last_level;

        if diff < 1 || diff > 3 {
            bad_levels += 1;
        } else {
            last_level = numbers[i];
        }

        if bad_levels > 1 {
            println!("inc: {:?}", &numbers);
            return false;
        }
    }
    return true;
}
fn increasing(numbers: &Vec<isize>) -> bool {
    let mut bad_levels = 0;
    for i in 0..numbers.len() - 1 {
        let j = i + 1;
        let diff = numbers[j] - numbers[i];
        if diff > 3 || diff < 1 {
            bad_levels += 1;
        }
    }
    bad_levels <= 1
}

fn decreasing(numbers: &Vec<isize>) -> bool {
    for i in 0..numbers.len() - 1 {
        let j = i + 1;
        let diff = numbers[i] - numbers[j];
        if diff > 3 || diff < 1 {
            return false;
        }
    }
    return true;
}

fn problem2_part1(contents: &str) {
    let mut safe: usize = 0;
    for line in contents.lines() {
        let v = line
            .split(" ")
            .map(|s| s.parse::<isize>().expect("we should've parsed a number."))
            .collect::<Vec<isize>>();
        if increasing_with_damping(&v) || decreasing_with_damping(&v) {
            safe += 1
        }
    }
    println!("safe: {:?}", safe)
}

// some id problem
fn problem1_part1(contents: &str) {
    let mut right: Vec<isize> = vec![];
    let mut left: Vec<isize> = vec![];
    for line in contents.lines() {
        let mut nums = line.trim().split("   ");
        left.push(nums.next().unwrap().trim().parse::<isize>().unwrap());
        right.push(nums.next().unwrap().trim().parse::<isize>().unwrap());
    }
    left.sort();
    right.sort();
    let mut total: isize = 0;
    for (l, r) in zip(&left, &right) {
        total += (l - r).abs()
    }
    println!("{:?}", total);
}

fn problem1_part2(contents: &str) {
    let default = 0;
    let mut left: HashMap<isize, isize> = HashMap::new();
    let mut right: Vec<isize> = vec![];
    for line in contents.lines() {
        let mut nums = line.trim().split("   ");
        let r = nums.next().unwrap().trim().parse::<isize>().unwrap();
        let l = nums.next().unwrap().trim().parse::<isize>().unwrap();
        right.push(r);
        let last = left.get(&l).unwrap_or(&default);
        left.insert(l, last + 1);
    }

    let mut total: isize = 0;
    for r in right.iter() {
        let l = left.get(r).unwrap_or(&default);
        total += r * l;
    }
    println!("{:?}", total);
}
