use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

pub fn main() {
    let filepath = "./inputs/input5.txt";
    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    parse_(&contents);
}

fn parse_(contents: &str) {
    let mut graph = HashMap::<usize, HashSet<usize>>::new();
    let mut lines = contents.lines();
    let mut sum = 0;
    loop {
        if let Some(line) = lines.next() {
            let trimmed_line = line.trim();

            if trimmed_line.len() == 0 {
                break;
            }

            let mut itr = trimmed_line
                .split('|')
                .map(|s| s.parse::<usize>().expect("There should be an int."));

            let k = itr.next().expect("There should be an integer");
            let v = itr.next().expect("There should be an integer");

            match graph.get_mut(&k) {
                None => {
                    let mut s = HashSet::new();
                    s.insert(v);
                    graph.insert(k, s);
                }
                Some(neighbors) => {
                    neighbors.insert(v);
                }
            }
        } else {
            panic!("Should of exit the loop at blank line");
        }
    }

    loop {
        if let Some(line) = lines.next() {
            let trimmed_line = line.trim();
            let mut pages = trimmed_line
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let ok = sort_(&graph, &mut pages);
            let n = pages.len();
            if ok {
                sum += pages[n / 2];
            }
            println!("{:?}: {:?} {:?}", pages, ok, pages[n / 2]);
        } else {
            break;
        }
    }
    println!("{:?}", sum);
}

fn check_(rules: &HashMap<usize, HashSet<usize>>, pages: &Vec<usize>) -> bool {
    // 75, 47, 61, 53, 29
    //     ^
    //
    // 75,97,47,61,53
    //    ^
    for i in 0..pages.len() {
        let k = pages[i];
        if let Some(v) = rules.get(&k) {
            for j in 0..i {
                if v.contains(&pages[j]) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn sort_(rules: &HashMap<usize, HashSet<usize>>, pages: &mut Vec<usize>) -> bool {
    // 75, 47, 61, 53, 29
    //     ^
    //
    // 75,97,47,61,53
    //    ^
    let mut reodered = false;
    for i in 0..pages.len() {
        let k = pages[i];
        if let Some(v) = rules.get(&k) {
            for j in 0..i {
                if v.contains(&pages[j]) {
                    pages.swap(i, j);
                    reodered = true
                }
            }
        }
    }
    return reodered;
}
