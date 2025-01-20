use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, isize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Robot {
    pos: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn new(pos: (isize, isize), velocity: (isize, isize)) -> Self {
        Self { pos, velocity }
    }

    fn update(&mut self, ncols: isize, nrows: isize) -> (isize, isize) {
        let (col, row) = self.pos;
        let (vcol, vrow) = self.velocity;

        let (mut urow, mut ucol) = (row + vrow, col + vcol);

        if urow < 0 {
            urow = ncols + urow;
        } else if urow >= ncols {
            urow = urow - ncols;
        }

        if ucol < 0 {
            ucol = nrows + ucol;
        } else if ucol >= nrows {
            ucol = ucol - nrows;
        }

        self.pos = (ucol, urow);
        (ucol, urow)
    }
}

struct Grid {
    robots: Vec<Robot>,
    ncols: isize,
    nrows: isize,
}

impl Grid {
    fn new(robots: Vec<Robot>, ncols: isize, nrows: isize) -> Self {
        Self {
            robots,
            nrows,
            ncols,
        }
    }

    fn update(&mut self) -> usize {
        let mut h: HashSet<(isize, isize)> = HashSet::new();
        for robot in self.robots.iter_mut() {
            let p = robot.update(self.nrows, self.ncols);
            h.insert(p);
        }
        h.len()
    }

    fn mid(&self) -> (isize, isize) {
        (self.ncols / 2, self.nrows / 2)
    }

    fn cuadrants(&self) -> usize {
        let mut upper_left = 0;
        let mut upper_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        let mid = self.mid();

        println!("mid {:?}", mid);

        for robot in self.robots.iter() {
            if robot.pos.0 < mid.0 && robot.pos.1 < mid.1 {
                println!("ul: {:?}", robot);
                upper_left += 1;
            } else if robot.pos.0 > mid.0 && robot.pos.1 < mid.1 {
                println!("bl: {:?}", robot);
                upper_right += 1;
            } else if robot.pos.0 < mid.0 && robot.pos.1 > mid.1 {
                println!("ur: {:?}", robot);
                bottom_left += 1;
            } else if robot.pos.0 > mid.0 && robot.pos.1 > mid.1 {
                println!("br: {:?}", robot);
                bottom_right += 1;
            }
        }
        println!(
            "{} {} {} {}",
            upper_left, upper_right, bottom_left, bottom_right
        );
        upper_left * bottom_left * bottom_right * upper_right
    }

    fn display(&self) -> Vec<Vec<usize>> {
        let mut grid = Vec::with_capacity(self.nrows as usize);

        for _ in 0..self.nrows {
            grid.push(Vec::from_iter((0..self.ncols).map(|_| 0)));
        }

        for robot in self.robots.iter() {
            let (col, row) = robot.pos;
            grid[row as usize][col as usize] += 1;
        }

        grid
    }

    fn print(&self, grid: Vec<Vec<usize>>) {
        let mid = self.mid();
        for row in 0..self.nrows {
            let mut line = String::new();

            if row == mid.1 {
                for _ in 0..self.ncols {
                    print!("");
                }
                print!("\n");
                continue;
            }

            for col in 0..self.ncols {
                let n = grid[row as usize][col as usize];
                if n == 0 && col as isize != mid.0 {
                    line += ".";
                } else if col as isize == mid.0 {
                    line += " ";
                } else {
                    line += &n.to_string();
                }
            }
            println!("{}", line);
        }
    }
}

fn main() {
    let filepath = "./inputs/input14.txt";

    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    let robots = parse(&contents);
    let nrobots = robots.len();
    println!("there are {}", nrobots);
    let mut g = Grid::new(robots, 101, 103);
    let mut itrs = 0;
    loop {
        let r = g.update();
        if r == nrobots {
            itrs = itrs + 1;
            println!("iters: {}", itrs);
            break;
        }
        itrs += 1;
    }
    println!("{}", g.cuadrants());
    g.print(g.display());
}

fn parse(content: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in content.lines() {
        let idx = line.find("p=").expect("Missing pattern.");
        let space = line.find(" ").expect("Missing comma.");

        let a: Vec<isize> = line[idx + "p=".len()..space]
            .split(",")
            .map(|s| s.parse::<isize>().expect("number"))
            .collect();

        let idx = line.find("v=").expect("Missing pattern.");
        let b: Vec<isize> = line[idx + "v=".len()..]
            .split(",")
            .map(|s| s.parse::<isize>().expect("number"))
            .collect();

        robots.push(Robot::new((a[0], a[1]), (b[0], b[1])))
    }
    robots
}
