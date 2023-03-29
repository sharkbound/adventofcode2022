use ndarray::Array2;
use rustutils::io::FileBuilder;
use rustutils::logging::DebugLog;
use rustutils::map_to::MapToExt;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn at_edge(&self, bounds: [usize; 2]) -> bool {
        self.x == 0 || self.x == bounds[1] || self.y == 0 || self.y == bounds[0]
    }

    fn as_usize_array_yx(&self) -> [usize; 2] {
        [self.y, self.x]
    }

    fn as_i32_array_yx(&self) -> [i32; 2] {
        [self.y as i32, self.x as i32]
    }
}

#[derive(Debug, Copy, Clone)]
struct Offset {
    x: i32,
    y: i32,
}

static ALLOWED_OFFSETS: [Offset; 4] = [
    Offset { x: -1, y: 0 },
    Offset { x: 1, y: 0 },
    Offset { x: 0, y: -1 },
    Offset { x: 0, y: 1 },
];

static START_INT: i32 = -1;
static END_INT: i32 = -2;

#[derive(Debug)]
struct PointData {
    pub point: Point,
    pub height: i32,
}

fn char_heightmap_to_i32_height(heightmap: &Array2<char>) -> Array2<i32> {
    let i32_heightmap = heightmap.map(|x| match *x {
        'S' => START_INT,
        'E' => END_INT,
        val => val as i32 - 'a' as i32,
    });
    i32_heightmap
}

pub struct Day12part1 {
    file: PathBuf,
    heightmap: Array2<char>,
    i32_heightmap: Array2<i32>,
}

enum BoundsCheck {
    Enabled,
    Disabled,
}
impl Day12part1 {
    pub fn new(input_path: PathBuf) -> Self {
        Self {
            file: input_path,
            heightmap: Array2::from_elem([0, 0], ' '),
            i32_heightmap: Array2::from_elem([0, 0], 0),
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

    fn _find_char(&self, find_char: char) -> Option<PointData> {
        self.heightmap.indexed_iter().find_map(|((y, x), c)| {
            if *c == find_char {
                Some(PointData {
                    point: Point::new(x, y),
                    height: self.i32_heightmap[[y, x]],
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

    fn create_point(&self, x: i32, y: i32) -> Option<Point> {
        let point = match (usize::try_from(x), usize::try_from(y)) {
            (Ok(x), Ok(y)) => Point::new(x, y),
            _ => return None,
        };

        let (bounds_y, bounds_x) = self.i32_heightmap.shape().map_to(|x| (x[0], x[1]));
        if point.x >= bounds_x || point.y >= bounds_y {
            return None;
        }

        Some(point)
    }

    fn neighbors(&self, point: Point) -> Vec<PointData> {
        let current_height = self.i32_heightmap[point.as_usize_array_yx()];
        let mut neighbors = Vec::new();
        for offset in ALLOWED_OFFSETS.iter() {
            let [x, y] = point.map_to(|Point { x, y }| [x as i32 + offset.x, y as i32 + offset.y]);

            match self.create_point(x, y) {
                Some(point) => {
                    let height = self.i32_heightmap[point.as_usize_array_yx()];
                    if (height - current_height).abs() > 1 {
                        continue;
                    }
                    neighbors.push(PointData { height, point });
                }
                None => continue,
            }
        }
        neighbors
    }

    pub fn solve(&mut self) {
        self.heightmap = self.parse();
        self.i32_heightmap = char_heightmap_to_i32_height(&self.heightmap);
        self.i32_heightmap.debug();
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
