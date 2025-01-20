use aoc_toolkit::{Direction, Grid, Vec2};
use std::{collections::HashSet, fs, usize};

pub fn main() {
    let filepath = "./inputs/input6.txt";
    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    let mut g = grid(&contents);
    let visited = path_finding(&mut g);

    let mut s = 0;
    for v in visited.iter() {
        if cycle_checking(&mut grid(&contents), *v) {
            s += 1;
        };
    }
    println!("{:?}", s);
}

fn grid(contents: &str) -> Grid<char> {
    Grid::parse_char(contents, &|x| x)
}

pub fn print(g: &Grid<char>) {
    for row in g.get_tiles() {
        for val in row {
            print!("{val}");
        }
        println!();
    }
}
fn try_from(value: char) -> Result<Direction, &'static str> {
    match value {
        '^' => Ok(Direction::Up),
        '>' => Ok(Direction::Left),
        'v' => Ok(Direction::Down),
        '<' => Ok(Direction::Left),
        _ => Err("Shit."),
    }
}

fn path_finding(g: &mut Grid<char>) -> HashSet<Vec2> {
    let mut visited = HashSet::new();
    let (c, mut current) = g
        .find(|c, _v| match *c {
            '^' | '>' | 'v' | '<' => true,
            _ => false,
        })
        .unwrap();

    let mut dir = try_from(*c).unwrap();

    loop {
        let dir_vec = dir.delta();
        let next = current + dir_vec;

        if let Some('#') = g.get(&next) {
            dir = dir.rotate90();
        } else if g.is_edge(&next) {
            visited.insert(next.clone());
            g.set(&next, 'X').unwrap();
            break;
        } else {
            visited.insert(current.clone());
            current = next;
        }
    }
    visited
}

fn cycle_checking(g: &mut Grid<char>, obstacle: Vec2) -> bool {
    let mut obstacles = HashSet::new();
    let (c, mut current) = g
        .find(|c, _v| match *c {
            '^' | '>' | 'v' | '<' => true,
            _ => false,
        })
        .unwrap();

    let mut dir = try_from(*c).unwrap();

    if obstacle == current {
        return false;
    } else {
        g.set(&obstacle, '#').unwrap();
    }

    loop {
        let dir_vec = dir.delta();
        let next = current + dir_vec;

        if obstacles.get(&(dir.clone(), next.clone())).is_some() {
            return true;
        }

        if let Some('#') = g.get(&next) {
            obstacles.insert((dir.clone(), next.clone()));
            dir = dir.rotate90();
        } else if g.is_edge(&next) {
            g.set(&next, 'X').unwrap();
            break;
        } else {
            current = next;
            g.set(&current, 'X').unwrap()
        }
    }
    false
}

fn path_finding_v1(g: &mut Grid<char>) -> usize {
    let mut block_by_col = HashSet::new();
    let mut block_by_row = HashSet::new();
    let mut loops = 0;
    let (c, mut current) = g
        .find(|c, _v| match *c {
            '^' | '>' | 'v' | '<' => true,
            _ => false,
        })
        .unwrap();

    let mut dir = try_from(*c).unwrap();

    loop {
        let dir_vec = dir.delta();
        let next = current + dir_vec;

        if let Some('#') = g.get(&next) {
            match dir {
                Direction::Up | Direction::Down => block_by_col.insert((next.0, dir.clone())),
                Direction::Left | Direction::Right => block_by_row.insert((next.1, dir.clone())),
                _ => panic!("shit"),
            };
            println!("blocks: {:?}, {:?}", block_by_col, block_by_row);
            dir = dir.rotate90();
        } else if g.is_edge(&next) {
            let d = dir.rotate90();
            match dir {
                Direction::Down | Direction::Up => {
                    if block_by_row.get(&(current.1, d)).is_some() {
                        loops += 1;
                        g.set(&next, 'O').unwrap();
                    }
                }
                Direction::Left | Direction::Right => {
                    if block_by_col.get(&(current.0, d)).is_some() {
                        loops += 1;
                        g.set(&next, 'O').unwrap();
                    }
                }
                _ => panic!("shit"),
            }
            break;
        } else {
            let d = dir.rotate90();
            match dir {
                Direction::Down | Direction::Up => {
                    if block_by_row.get(&(current.1, d)).is_some() {
                        loops += 1;
                        g.set(&next, 'O').unwrap();
                    }
                }
                Direction::Left | Direction::Right => {
                    if block_by_col.get(&(current.0, d)).is_some() {
                        loops += 1;
                        g.set(&next, 'O').unwrap();
                    }
                }
                _ => panic!("shit"),
            }
            current = next;
        }
        if let Some('.') = g.get(&current) {
            g.set(&current, 'X').unwrap()
        }
    }
    loops
}
