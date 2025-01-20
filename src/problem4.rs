use aoc_toolkit::{Direction, Grid, Vec2};
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

pub fn main() {
    let filepath = "./inputs/input4.txt";
    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    println!("{:?}", xmas(&contents));
    println!("{:?}", mas(&contents));
}

fn searching_for(c: char) -> char {
    match c {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("dude, wtf."),
    }
}

fn mas(contents: &str) -> usize {
    let mut found = 0;
    let g = Grid::parse_char(contents, &|x| x);
    for (c, pos) in g.get_tiles_flat() {
        if *c == 'A' {
            // 0 is UpRight, 1 is DownLeft, 2 is UpLeft and 3 is DownRight
            // The idea being you should get "smsm" or "mssm" or "msms" or "smms"
            let mut mybox = ['x'; 4];
            for (c, d, _v) in g.adjacent_neighbors(pos) {
                match d {
                    Direction::Up | Direction::Down | Direction::Left | Direction::Right => {
                        continue
                    }
                    Direction::UpRight => mybox[0] = c,
                    Direction::UpLeft => mybox[2] = c,
                    Direction::DownRight => mybox[3] = c,
                    Direction::DownLeft => mybox[1] = c,
                }
            }
            let s = mybox.iter().collect::<String>();
            match &s[..] {
                "SMSM" | "MSSM" | "SMMS" | "MSMS" => found += 1,
                _ => {}
            }
        }
    }
    found
}

fn xmas(contents: &str) -> usize {
    let mut found = 0;
    let g = Grid::parse_char(contents, &|x| x);
    for (c, v) in g.get_tiles_flat() {
        if *c == 'X' {
            let mut queue = VecDeque::new();
            for d in Direction::adjacent() {
                queue.push_back((v, d, searching_for(*c)));
            }
            loop {
                if queue.is_empty() {
                    break;
                }
                let (pos, dir, searching) = queue.pop_front().unwrap();

                for (c, d, v) in g.adjacent_neighbors(pos) {
                    if searching == 'S' && searching == c && d == dir {
                        found += 1;
                    } else if searching == c && d == dir {
                        queue.push_back((v, d, searching_for(c)));
                        println!("{:?}", c);
                    }
                }
            }
        }
    }
    found
}
