use std::{
    collections::{HashMap, VecDeque},
    fs, isize,
};

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_vector(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn opposite_point(&self, other: &Self) -> Self {
        let v = self.manhattan_vector(other);
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Point>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: vec![],
            antennas: HashMap::new(),
        }
    }

    fn parse(&mut self, contents: &str) {
        for (nrow, line) in contents.lines().enumerate() {
            let mut row = vec![];

            for (ncol, c) in line.trim().chars().enumerate() {
                row.push(c);
                if c == '.' {
                    continue;
                }
                if let Some(antenna) = self.antennas.get_mut(&c) {
                    antenna.push(Point::new(ncol as isize, nrow as isize));
                } else {
                    self.antennas
                        .insert(c, vec![Point::new(ncol as isize, nrow as isize)]);
                }
            }

            self.data.push(row);
        }
    }

    fn size(&self) -> (isize, isize) {
        (self.data[0].len() as isize, self.data.len() as isize)
    }

    fn compute(&mut self) -> usize {
        let size = self.size();
        let mut count = 0;
        for (c, antenna) in self.antennas.iter() {
            println!("{:?}, {:?}", c, &antenna);
            for i in 0..antenna.len() {
                for j in 0..antenna.len() {
                    if i == j {
                        continue;
                    }

                    let a = &antenna[i];
                    let b = &antenna[j];

                    let v = a.manhattan_vector(b);
                    let mut scale = 0;

                    loop {
                        let p = Point::new(a.x + scale * v.x, a.y + scale * v.y);

                        if p.x < 0 || p.x >= size.0 || p.y < 0 || p.y >= size.1 {
                            println!("{:?} {:?} {:?}", a, b, p);
                            break;
                        }

                        if self.data[p.y as usize][p.x as usize] != '#' {
                            self.data[p.y as usize][p.x as usize] = '#';
                            println!("+1 {:?} {:?} {:?}", a, b, p);
                            count += 1;
                        }
                        scale += 1;
                    }
                }
            }
        }
        count
    }

    fn print(&self) -> String {
        let s: Vec<String> = self.data.iter().map(|v| v.iter().collect()).collect();
        s.join("\n")
    }
}

#[cfg(test)]
mod test_points {
    use crate::Point;

    #[test]
    fn test_points() {
        let a = Point::new(4, 3);
        let b = Point::new(5, 5);
        let c = Point::new(8, 4);

        assert_eq!(a.opposite_point(&b), Point::new(3, 1));
        assert_eq!(b.opposite_point(&a), Point::new(6, 7));
        assert_eq!(a.opposite_point(&c), Point::new(0, 2));
        assert_eq!(b.opposite_point(&c), Point::new(2, 6));

        // Should be out of bounds...
        assert_eq!(c.opposite_point(&a), Point::new(12, 5));
    }
}

fn main() {
    let contents = fs::read_to_string("./inputs/input8.txt").expect("FILE");
    let mut g = Grid::new();
    g.parse(&contents);
    println!("{:?}", g.compute());
    println!("{:}", g.print());
}
