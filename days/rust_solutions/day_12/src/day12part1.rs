use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Day12part1 {
    file: File,
    heightmap: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Point(pub usize, pub usize);

impl Point {
    pub fn height(&self, grid: &Vec<Vec<char>>) -> Option<u32> {
        grid.get(self.0)
            .and_then(|x| x.get(self.1))
            .and_then(|x| (*x as u32).checked_sub('a' as u32))
    }
}

static ALLOWED_OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Point {
    pub fn neighbors(&self) -> Vec<Self> {
        ALLOWED_OFFSETS.iter()
            .filter_map(|(y, x)|
                if (self.0 as i32 + y) < 0 || (self.1 as i32 + x) < 0 {
                    None
                } else {
                    Some(Point((self.0 as i32 + y) as usize, (self.1 as i32 + x) as usize))
                }
            ).collect()
    }
}

#[derive(Debug)]
struct CharPos {
    pub point: Point,
    pub chr: char,
}


trait ElevationExt {
    fn elevation(&self) -> u16;
    fn elevation_checked(&self) -> Option<u16>;
}


impl ElevationExt for char {
    fn elevation(&self) -> u16 {
        *self as u16 - 'a' as u16
    }

    fn elevation_checked(&self) -> Option<u16> {
        (*self as u16).checked_sub('a' as u16)
    }
}

impl Day12part1 {
    pub fn new(input_path: PathBuf) -> Self {
        let file = match File::options().read(true).open(input_path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Unable to read input file: {:?} !", e);
            }
        };
        Self { file, heightmap: Vec::new() }
    }

    pub fn parse(&self) -> Vec<Vec<char>> {
        BufReader::new(&self.file).lines().filter_map(|x| match x {
            Ok(line) => Some(line.chars().collect::<Vec<_>>()),
            Err(_) => panic!("Unable to read line from input file: {:?}", &self.file),
        }).collect::<Vec<_>>()
    }

    fn _find_char(&self, find_char: char) -> CharPos {
        let data = self.heightmap
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)| {
                match row.iter().position(|chr| chr == &find_char) {
                    Some(col_idx) => Some((row_idx, col_idx)),
                    None => None
                }
            }).nth(0).unwrap();

        CharPos {
            point: Point(data.0, data.1),
            chr: find_char,
        }
    }


    pub fn solve(&mut self) {
        self.heightmap = self.parse();
        let (start, end) = (self._find_char('S'), self._find_char('E'));
        println!("{:?} {:?}", start, Point(0, 0).height(&self.heightmap));
    }
}
/*
a is the lowest elevation, b is the next-lowest, and so on up to the highest elevation, z.

You'd like to reach E, but to save energy, you should do it in as few steps as possible.

During each step, you can move exactly one square up, down, left, or right.

To avoid needing to get out your climbing gear,
    the elevation of the destination square can be at most one higher than the
    elevation of your current square; that is, if your current elevation is m,
    you could step to elevation n, but not to elevation o.

(This also means that the elevation of the destination square can be much lower than the elevation of your current square.)
 */
