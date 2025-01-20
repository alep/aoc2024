use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub(crate) struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub(crate) fn new() -> Self {
        Self { data: vec![] }
    }

    pub(crate) fn parse(&mut self, contents: &str) {
        for (_nrow, line) in contents.lines().enumerate() {
            let mut row = vec![];
            for (_ncol, c) in line.trim().chars().enumerate() {
                row.push(c);
            }
            self.data.push(row);
        }
    }

    pub(crate) fn segment(&self, start: Point) -> HashSet<Point> {
        let mut q: VecDeque<Point> = VecDeque::new();
        let (len_x, len_y) = self.size();
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut area: HashSet<Point> = HashSet::new();

        let region = self.data[start.x as usize][start.y as usize];

        q.push_back(start);
        area.insert(start);

        loop {
            if q.is_empty() {
                break;
            }

            let p = q.pop_front().unwrap();
            for (dy, dx) in neighbors {
                let (i0, j0) = (p.x + dy, p.y + dx);

                if i0 < 0 || j0 < 0 || i0 >= len_x || j0 >= len_y {
                    continue;
                } else {
                    let r = self.data[i0 as usize][j0 as usize];
                    let p = Point::new(i0, j0);
                    if r == region && area.get(&p).is_none() {
                        area.insert(p);
                        q.push_back(p);
                    }
                }
            }
        }
        area
    }

    fn walls(&self, p: &Point) -> usize {
        let (i, j): (isize, isize) = (p.x as isize, p.y as isize);
        let region = self.data[p.x as usize][p.y as usize];

        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (len_x, len_y) = self.size();

        let mut nwall = 0;
        for (dy, dx) in neighbors {
            let (i0, j0) = (i + dy, j + dx);

            if i0 < 0 || j0 < 0 || i0 >= len_x || j0 >= len_y {
                nwall += 1;
            } else {
                let r = self.data[i0 as usize][j0 as usize];
                if r != region {
                    nwall += 1;
                }
            }
        }
        nwall
    }

    fn in_region(&self, p: Point, region: char) -> bool {
        let (i0, j0) = (p.x, p.y);
        let (len_x, len_y) = self.size();

        if i0 < 0 || j0 < 0 || i0 >= len_x || j0 >= len_y {
            false
        } else {
            let r = self.data[i0 as usize][j0 as usize];
            if r == region {
                true
            } else {
                false
            }
        }
    }

    fn corner(&self, p: &Point) -> usize {
        let mut ncorners = 0;
        let region = self.data[p.x as usize][p.y as usize];
        let neighbors = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        for (r, c) in neighbors {
            let row_neighbor = Point::new(p.x + r, p.y);
            let col_neighbor = Point::new(p.x, p.y + c);
            let diagonal_neighbor = Point::new(p.x + r, p.y + c);

            if !self.in_region(row_neighbor, region) && !self.in_region(col_neighbor, region) {
                ncorners += 1;
            }

            if self.in_region(row_neighbor, region)
                && self.in_region(col_neighbor, region)
                && !self.in_region(diagonal_neighbor, region)
            {
                ncorners += 1;
            }
        }
        ncorners
    }

    fn is_border(&self, p: &Point) -> bool {
        let (len_x, len_y) = self.size();
        let region = self.data[p.x as usize][p.y as usize];
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in neighbors {
            let (i0, j0) = (p.x + dy, p.y + dx);

            if i0 < 0 || j0 < 0 || i0 >= len_x || j0 >= len_y {
                return true;
            } else {
                let r = self.data[i0 as usize][j0 as usize];
                if r != region {
                    return true;
                }
            }
        }
        false
    }

    fn line_borders(&self, mut borders: HashSet<(Point, Point)>) -> usize {
        let (mut next, mut p) = borders
            .iter()
            .min_by(|(p0, p1), (q0, q1)| p0.cmp(q0))
            .unwrap();
        let region = self.data[p.x as usize][p.y as usize];
        borders.remove(&(next, p));

        let mut nlines = 0;
        let mut direction = "right";
        let mut last_direction = direction;
        loop {
            // find next, go right first
            println!("next: {:?}", next);
            if borders.is_empty() {
                break;
            }
            let candidate = if direction == "down" {
                Point::new(next.x + 1, next.y)
            } else if direction == "right" {
                Point::new(next.x, next.y + 1)
            } else if direction == "up" {
                Point::new(next.x - 1, next.y)
            } else {
                Point::new(next.x, next.y - 1)
            };

            if borders.get(&(candidate, p)).is_some() {
                let n = match direction {
                    "down" => Point::new(p.x + 1, p.y),
                    "up" => Point::new(p.x - 1, p.y),
                    "right" => Point::new(p.x, p.y + 1),
                    "left" => Point::new(p.x, p.y - 1),
                    _ => panic!("shouldn't happen"),
                };

                let (len_x, len_y) = self.size();
                let (i0, j0) = (n.x, n.y);
                let mut del = false;

                if 0 <= i0 && i0 < len_x && 0 <= j0 && j0 < len_y {
                    let r = self.data[i0 as usize][j0 as usize];
                    if r == region {
                        del = true;
                    }
                } else {
                    del = true;
                }

                if del {
                    borders.remove(&(candidate, p));
                    next = candidate;
                    if last_direction != direction {
                        nlines += 1;
                    }
                } else {
                    last_direction = direction;
                    direction = match direction {
                        "right" => "down",
                        "down" => "left",
                        "left" => "up",
                        "up" => "right",
                        _ => panic!("not expected"),
                    };
                }
            } else {
                last_direction = direction;
                direction = match direction {
                    "right" => "down",
                    "down" => "left",
                    "left" => "up",
                    "up" => "right",
                    _ => panic!("not expected"),
                };
            }
        }
        nlines
    }

    fn size(&self) -> (isize, isize) {
        (self.data.len() as isize, self.data[0].len() as isize)
    }

    pub(crate) fn compute2(&self) -> usize {
        let mut total = 0;
        let mut visited: HashSet<Point> = HashSet::new();

        let (nrows, ncols) = self.size();

        for i in 0..nrows {
            for j in 0..ncols {
                let p = Point::new(i, j);

                if visited.get(&p).is_some() {
                    continue;
                }

                let area = self.segment(p);
                visited = visited.union(&area).map(|p| *p).collect();

                let mut result = 0;
                for point in area.iter() {
                    result += self.corner(&point);
                }

                total += area.len() * result;
                println!(
                    "region: {}, area: {}, perimeter: {}",
                    self.data[p.x as usize][p.y as usize],
                    area.len(),
                    result
                );
            }
        }

        total
    }

    pub(crate) fn compute(&self) -> usize {
        let mut total = 0;
        let mut visited: HashSet<Point> = HashSet::new();

        let (nrows, ncols) = self.size();

        for i in 0..nrows {
            for j in 0..ncols {
                let p = Point::new(i, j);

                if visited.get(&p).is_some() {
                    continue;
                }

                let area = self.segment(p);
                visited = visited.union(&area).map(|p| *p).collect();

                let mut result = 0;
                for point in area.iter() {
                    result += self.walls(point);
                }
                total += area.len() * result;
                println!(
                    "region: {}, area: {}, perimeter: {}",
                    self.data[p.x as usize][p.y as usize],
                    area.len(),
                    result
                );
            }
        }

        total
    }

    pub(crate) fn print(&self) -> String {
        let s: Vec<String> = self.data.iter().map(|v| v.iter().collect()).collect();
        s.join("\n")
    }
}

pub fn main() {
    let filepath = "./inputs/input12.txt";

    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    let mut grid = Grid::new();
    grid.parse(&contents);
    println!("{}", grid.print());
    let result = grid.compute2();
    println!("{}", result);
}
