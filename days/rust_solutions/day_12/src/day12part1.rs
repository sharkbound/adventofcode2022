use ndarray::Array2;
use rustutils::io::FileBuilder;
use rustutils::logging::DebugLog;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn as_u2(&self) -> [usize; 2] {
        [self.x, self.y]
    }

    fn add(&self, y: i32, x: i32) -> Option<Point> {
        return if (self.x as i32 + y) < 0 {
            None
        } else if (self.y as i32 + x) < 0 {
            None
        } else {
            Some(Point::new(
                (self.x as i32 + y) as usize,
                (self.y as i32 + x) as usize,
            ))
        };
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

static ALLOWED_OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug)]
struct CharPos {
    pub point: Point,
    pub chr: char,
}

pub struct Day12part1 {
    file: PathBuf,
    heightmap: Array2<char>,
}

impl Day12part1 {
    pub fn new(input_path: PathBuf) -> Self {
        Self {
            file: input_path,
            heightmap: Array2::from_elem([0, 0], ' '),
        }
    }

    pub fn parse(&self) -> Array2<char> {
        let lines = FileBuilder::from_pathbuf(self.file.clone())
            .read()
            .buffered_reader()
            .unwrap()
            .lines()
            .flatten()
            .collect::<Vec<_>>();

        let mut arr = Array2::from_elem([lines.len(), lines[0].len()], ' ');
        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                arr[[y, x]] = c;
            }
        }
        arr
    }

    fn _find_char(&self, find_char: char) -> Option<CharPos> {
        self.heightmap.indexed_iter().find_map(|((y, x), c)| {
            if *c == find_char {
                Some(CharPos {
                    point: Point::new(x, y),
                    chr: *c,
                })
            } else {
                None
            }
        })
    }

    fn path_find(&self, start: &Point, should_stop: impl Fn(&Point) -> bool) -> Vec<Vec<Point>> {
        let mut path = vec![*start];
        let mut paths = Vec::new();
        paths
    }

    pub fn solve(&mut self) {
        self.heightmap = self.parse();
        self.heightmap.debug();
        let (start, end) = (self._find_char('S').unwrap(), self._find_char('E').unwrap());
        println!("start: {:?}, end: {:?}", start, end);
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
/*
reference implementations
https://www.reddit.com/r/adventofcode/comments/zjnruc/comment/j2du9yj/
https://www.reddit.com/r/adventofcode/comments/zjnruc/comment/j2tx1um/
 */
