use aoc_toolkit::{Direction, Grid, Vec2};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

#[derive(Debug, Clone)]
struct Step {
    height: usize,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.height)
    }
}

pub fn main() {
    let filepath = "./inputs/input10.txt";

    let contents = fs::read_to_string(filepath).expect("there should of been a file.");

    let g = grid(&contents);
    let mut s = 0;
    for head in trailheads(&g) {
        s += crawl(&g, head.1);
    }
    println!("{}", s);
}

fn grid(contents: &str) -> Grid<Step> {
    Grid::parse_char(contents, &|x| Step {
        height: x.to_string().parse::<usize>().expect("number!"),
    })
}

fn trailheads(grid: &Grid<Step>) -> impl Iterator<Item = (&Step, Vec2)> {
    grid.filter(|s, _v| s.height == 0)
}

fn crawl(grid: &Grid<Step>, head: Vec2) -> usize {
    let mut count = 0;

    let mut stack: VecDeque<Vec2> = VecDeque::new();

    stack.push_back(head);

    loop {
        // S is & to step
        let v = match stack.pop_back() {
            None => break,
            Some(v) => v,
        };

        for (step, _dir, vec2) in grid.cardinal_neighbors(v) {
            let cur_step = grid.get(&v).unwrap();

            if step.height > cur_step.height && (step.height - cur_step.height) == 1 {
                stack.push_back(vec2);

                if step.height == 9 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn print(g: &Grid<Step>) {
    for row in g.get_tiles() {
        for val in row {
            print!("{val}");
        }
        println!();
    }
}
