use aoc_toolkit::{Direction, Grid, Vec2};
use std::{collections::HashSet, fs, usize};

pub fn main() {
    let filepath = "./inputs/test_input8.txt";
    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    let mut g = grid(&contents);
    print(&g);
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
