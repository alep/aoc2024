use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Num {
    Split(usize, usize),
    Single(usize),
}

pub fn main() {
    let filepath = "./inputs/test_input11.txt";

    let contents = fs::read_to_string(filepath).expect("there should of been a file.");

    let data: Vec<usize> = contents
        .trim()
        .split(' ')
        .flat_map(|s| s.parse::<usize>())
        .collect();

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut sum = 0;
    for num in data.iter() {
        sum += apply(*num, 25, &mut cache);
    }
    println!("{:?}", sum)
}

fn ndigits(n: usize) -> usize {
    f64::floor(f64::log10(n as f64) + 1.) as usize
}

fn check_digits(n: usize) -> bool {
    ndigits(n) % 2 == 0
}

fn split_digis(n: usize) -> (usize, usize) {
    let itrs = ndigits(n) / 2;
    let mut p = n;
    let mut m = 0;

    for i in 0..itrs {
        m += (p % 10) * 10_usize.pow(i as u32);

        p = p / 10;
    }

    (p, m)
}

fn apply_rules(data: Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    for n in data.iter() {
        if *n == 0 {
            result.push(1)
        } else if check_digits(*n) {
            let (p, m) = split_digis(*n);
            result.push(p);
            result.push(m);
        } else {
            result.push(n * 2024);
        }
    }
    result
}

fn apply(k: usize, n: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return 1;
    }

    if let Some(res) = cache.get(&(k, n)) {
        return *res;
    }

    let result = if k == 0 {
        apply(1, n - 1, cache)
    } else if check_digits(k) {
        let (p, m) = split_digis(k);
        apply(p, n - 1, cache) + apply(m, n - 1, cache)
    } else {
        apply(k * 2024, n - 1, cache)
    };
    cache.insert((k, n), result);
    result
}
