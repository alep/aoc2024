use std::{
    fmt::{self, Formatter},
    fs, usize,
};

fn main() {
    let contents = fs::read_to_string("./inputs/input9.txt").expect("FILE");
    let mut disk = Disk::new(contents.trim());
    println!("{}", disk);
    disk.refrag();
    println!("{}", disk);
    println!("{}", disk.checksum());
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Block {
    File { id: usize, start: usize, len: usize },
    Free { start: usize, len: usize },
}

impl Block {
    fn is_free(&self) -> bool {
        match self {
            Block::Free { .. } => true,
            _ => false,
        }
    }

    fn size(&self) -> usize {
        match self {
            Block::Free { start: _start, len } => *len,
            Block::File {
                id: _id,
                start: _start,
                len,
            } => *len,
        }
    }

    fn start(&self) -> usize {
        match self {
            Block::Free { start, len: _len } => *start,
            Block::File {
                id: _id,
                start,
                len: _len,
            } => *start,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Block::Free { .. } => write!(f, "."),
            Block::File { id, .. } => write!(f, "{}", id),
        }
    }
}

#[derive(Debug)]
struct Disk {
    data: Vec<Block>,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for elem in self.data.iter() {
            write!(f, "{}", elem)?
        }
        Ok(())
    }
}

impl Disk {
    fn new(contents: &str) -> Self {
        let mut id = 0;
        let mut data: Vec<Block> = vec![];
        let mut data_index = 0;
        for (index, c) in contents.chars().enumerate() {
            let size = c.to_string().parse::<usize>().expect("should be a digit");

            let block = match index % 2 {
                0 => {
                    let block = Block::File {
                        id,
                        start: data_index,
                        len: size,
                    };
                    id += 1;
                    block
                }
                _ => Block::Free {
                    start: data_index,
                    len: size,
                },
            };
            for _ in 0..size {
                data.push(block);
                data_index += 1;
            }
        }
        Self { data }
    }

    fn defrag(&mut self) {
        let mut free_ptr = 0;
        let mut block_ptr = self.data.len() - 1;

        loop {
            if free_ptr >= block_ptr {
                break;
            }

            if !self.data[free_ptr].is_free() {
                free_ptr += 1;
                continue;
            }

            if self.data[block_ptr].is_free() {
                block_ptr -= 1;
                continue;
            }

            self.data.swap(free_ptr, block_ptr)
        }
    }

    fn refrag(&mut self) {
        let mut file_block_ptr = self.data.len() - 1;

        loop {
            if 0 >= file_block_ptr {
                break;
            }

            let block = self.data[file_block_ptr];
            let file_block = match block {
                Block::Free { start: _start, len } => {
                    file_block_ptr -= len;
                    continue;
                }
                Block::File { .. } => block,
            };

            let mut free_block_ptr = 0;

            loop {
                if free_block_ptr >= file_block_ptr {
                    break;
                }

                let block = self.data[free_block_ptr];

                let free_block = match block {
                    Block::File {
                        id: _id,
                        start: _start,
                        len,
                    } => {
                        free_block_ptr += len;
                        continue;
                    }
                    Block::Free { .. } => block,
                };

                if free_block.size() >= file_block.size() {
                    for i in 0..file_block.size() {
                        let a = free_block.start() + i;
                        let b = file_block.start() + i;
                        self.data.swap(a, b);
                    }

                    let remainder = free_block.size() - file_block.size();
                    let start = free_block.start() + file_block.size();
                    for i in 0..remainder {
                        let idx = start + i;
                        self.data[idx] = Block::Free {
                            start,
                            len: remainder,
                        };
                    }

                    break;
                } else {
                    free_block_ptr += free_block.size();
                }
            }

            match file_block_ptr.checked_sub(file_block.size()) {
                Some(v) => file_block_ptr = v,
                None => break,
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut check = 0;
        for (idx, elem) in self.data.iter().enumerate() {
            match elem {
                Block::Free { .. } => {}
                Block::File { id, .. } => {
                    check += idx * id;
                }
            }
        }
        check
    }
}
