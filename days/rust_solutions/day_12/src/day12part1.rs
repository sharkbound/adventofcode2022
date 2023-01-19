use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::path::PathBuf;

pub struct Day12part1 {
    file: File,
    heightmap: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone)]
struct Point(pub usize, pub usize);
impl Point {
    fn try_add(&self, y: i32, x: i32) -> Option<Point> {
        return if (self.0 as i32 + y) < 0 {
            None
        } else if (self.1 as i32 + x) < 0 {
            None
        } else {
            Some(Point(
                (self.0 as i32 + y) as usize,
                (self.1 as i32 + x) as usize,
            ))
        };
    }
}

static ALLOWED_OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct CharPos {
    pub point: Point,
    pub chr: char,
}

trait HeightMap {
    fn height(&self, y: usize, x: usize) -> Option<u32>;
    fn height_at_point(&self, point: Point) -> Option<u32>;
    fn valid_moves(&self, point: Point) -> Vec<(Point, u32)>;
    fn neighbors(&self, point: Point) -> Vec<Point>;
}

impl HeightMap for Vec<Vec<char>> {
    fn height(&self, y: usize, x: usize) -> Option<u32> {
        match self.get(y).and_then(|v| v.get(x)) {
            Some('S') => Some(0),
            Some('E') => Some(25),
            Some(chr) => (*chr as u32).checked_sub('a' as u32),
            None => None,
        }
    }

    fn height_at_point(&self, point: Point) -> Option<u32> {
        self.height(point.0, point.1)
    }

    fn valid_moves(&self, point: Point) -> Vec<(Point, u32)> {
        let current_height = match self.height_at_point(point) {
            None => {
                return vec![];
            }
            Some(x) => x,
        };

        self.neighbors(point)
            .iter()
            .filter_map(|p| match self.height_at_point(*p) {
                None => None,
                Some(h) => {
                    if h as i32 - current_height as i32 > 1 {
                        None
                    } else {
                        Some((*p, h))
                    }
                }
            })
            .collect()
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        ALLOWED_OFFSETS
            .iter()
            .filter_map(|(y, x)| point.try_add(*y, *x))
            .collect()
    }
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
        Self {
            file,
            heightmap: Vec::new(),
        }
    }

    pub fn parse(&self) -> Vec<Vec<char>> {
        BufReader::new(&self.file)
            .lines()
            .filter_map(|x| match x {
                Ok(line) => Some(line.chars().collect::<Vec<_>>()),
                Err(_) => panic!("Unable to read line from input file: {:?}", &self.file),
            })
            .collect::<Vec<_>>()
    }

    fn _find_char(&self, find_char: char) -> CharPos {
        let data = self
            .heightmap
            .iter()
            .enumerate()
            .filter_map(
                |(row_idx, row)| match row.iter().position(|chr| chr == &find_char) {
                    Some(col_idx) => Some((row_idx, col_idx)),
                    None => None,
                },
            )
            .nth(0)
            .unwrap();

        CharPos {
            point: Point(data.0, data.1),
            chr: find_char,
        }
    }

    fn path_find(&self, start: Point, end: Point)  {
        let path = vec![];


    }

    pub fn solve(&mut self) {
        self.heightmap = self.parse();

        let (start, end) = (self._find_char('S'), self._find_char('E'));
        println!("{:?} {:?}", start, self.heightmap.valid_moves(Point(0, 0)));
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
