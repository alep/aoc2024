use aoc_toolkit::{Direction, Grid, Vec2};
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env::current_dir;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::usize;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Wall,
    Reindeer,
    End,
    Up,
    Down,
    Left,
    Right,
}

impl Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c = match self {
            CellType::Empty => '.',
            CellType::Reindeer => 'S',
            CellType::Wall => '#',
            CellType::End => 'E',
            CellType::Up => '^',
            CellType::Down => 'V',
            CellType::Left => '<',
            CellType::Right => '>',
        };
        write!(f, "{c}")
    }
}

impl From<char> for CellType {
    fn from(s: char) -> Self {
        match s {
            '.' => CellType::Empty,
            '#' => CellType::Wall,
            'S' => CellType::Reindeer,
            'E' => CellType::End,
            '^' => CellType::Up,
            'V' => CellType::Down,
            '<' => CellType::Left,
            '>' => CellType::Right,
            _ => panic!("Unknown char."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GridCell {
    cell_type: CellType,
    visited_directions: HashSet<Direction>,
    visited: bool,
}

pub fn main() {
    let filepath = "./inputs/input16.txt";

    let contents = fs::read_to_string(filepath).expect("there should of been a file.");

    let mut g = grid(&contents);
    print(&g);
    let (distance, prev) = dijkstra(&g);

    let (_reindeer, start) = g
        .find(|c, _v| match c {
            CellType::Reindeer => true,
            _ => false,
        })
        .unwrap();

    let (_end, end) = g
        .find(|c, _v| match c {
            CellType::End => true,
            _ => false,
        })
        .unwrap();

    let mut m = usize::MAX;
    let mut finish: Option<(Vec2, Direction)> = None;
    for d in Direction::cardinal() {
        match distance.get(&(end, d.clone())) {
            Some(dis) => {
                if *dis < m {
                    m = *dis;
                    finish = Some((end, d.clone()));
                }
            }
            None => {}
        }
    }
    println!(">>>>> finish: {:?}, {:?}", finish, m);
    path(
        &mut g,
        (start, Direction::Right),
        finish.unwrap(),
        distance,
        prev,
    );
    print(&g);
}

fn grid(contents: &str) -> Grid<CellType> {
    Grid::parse_char(contents, &|x| x.into())
}

fn dijkstra(
    grid: &Grid<CellType>,
) -> (
    HashMap<(Vec2, Direction), usize>,
    HashMap<(Vec2, Direction), (Vec2, Direction)>,
) {
    let (_reindeer, start) = grid
        .find(|c, _v| match c {
            CellType::Reindeer => true,
            _ => false,
        })
        .unwrap();

    let mut distance: HashMap<(Vec2, Direction), usize> = HashMap::new();
    let mut prev: HashMap<(Vec2, Direction), (Vec2, Direction)> = HashMap::new();
    let mut queue: VecDeque<(Vec2, Direction, usize)> = VecDeque::new();

    distance.insert((start, Direction::Right), 0);
    prev.insert((start, Direction::Right), (start, Direction::Right));
    queue.push_back((start, Direction::Right, 0));

    loop {
        if queue.is_empty() {
            break;
        }

        let u = {
            let mut min_value = usize::MAX;
            let mut curr_cell: Option<(Vec2, Direction, usize)> = None;
            let mut last_index: Option<usize> = None;
            for (index, cell_vec) in queue.iter().enumerate() {
                let d = *distance
                    .get(&(cell_vec.0, cell_vec.1.clone()))
                    .unwrap_or(&usize::MAX);
                if d <= min_value {
                    min_value = d;
                    curr_cell = Some(cell_vec.clone());
                    last_index = Some(index);
                }
            }
            queue.remove(last_index.unwrap());

            curr_cell.unwrap()
        };

        if *distance.get(&(u.0, u.1.clone())).unwrap_or(&usize::MAX) < u.2 {
            continue;
        }

        for dir in Direction::cardinal() {
            let current_dir = u.1.clone();
            if dir == current_dir {
                continue;
            }

            let d = *distance.get(&(u.0, dir.clone())).unwrap_or(&usize::MAX);
            if d > u.2 + 1000 {
                distance.insert((u.0, dir.clone()), u.2 + 1000);
                queue.push_back((u.0, dir.clone(), u.2 + 1000));
            }
        }

        let delta_vector = Direction::delta(&u.1);
        let vec2 = u.0 + delta_vector;

        match grid.get(&vec2) {
            Some(CellType::Empty) => {
                let alt = u.2 + 1;
                if alt < *distance.get(&(vec2, u.1.clone())).unwrap_or(&usize::MAX) {
                    distance.insert((vec2, u.1.clone()), alt);
                    prev.insert((vec2, u.1.clone()), (u.0, u.1.clone()));
                    queue.push_back((vec2, u.1.clone(), alt));
                }
            }
            Some(CellType::End) => {
                let alt = u.2 + 1;
                if alt < *distance.get(&(vec2, u.1.clone())).unwrap_or(&usize::MAX) {
                    distance.insert((vec2, u.1.clone()), alt);
                    prev.insert((vec2, u.1.clone()), (u.0, u.1.clone()));
                }
            }
            _ => {}
        }
    }

    (distance, prev)
}
fn path(
    g: &mut Grid<CellType>,
    start: (Vec2, Direction),
    end: (Vec2, Direction),
    distance: HashMap<(Vec2, Direction), usize>,
    prev: HashMap<(Vec2, Direction), (Vec2, Direction)>,
) {
    let mut last = end;
    loop {
        last = match prev.get(&last) {
            None => break,
            Some(v) if *v == start => break,

            Some((vec2, dir)) => {
                let value = distance.get(&last).unwrap();
                let c = match dir {
                    Direction::Up => '^',
                    Direction::Down => 'V',
                    Direction::Right => '>',
                    Direction::Left => '<',
                    _ => panic!("dude, fuck off. there shouldn't be other directions."),
                };
                let _ = g.set(vec2, c.into());
                (*vec2, dir.clone())
            }
        }
    }
}

fn print(g: &Grid<CellType>) {
    for row in g.get_tiles() {
        for val in row {
            print!("{}", val);
        }
        println!();
    }
}
