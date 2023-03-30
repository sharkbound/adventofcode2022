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

static START_HEIGHT: i32 = -1;
static END_HEIGHT: i32 = -2;
static START_CHAR: char = 'S';
static END_CHAR: char = 'E';

#[derive(Debug, Copy, Clone)]
struct PointData {
    pub point: Point,
    pub height: i32,
}

impl PointData {
    fn as_char(&self) -> char {
        match self.height {
            i if i == START_HEIGHT => 'S',
            i if i == END_HEIGHT => 'E',
            i => char::from_digit(i as u32 + 'a'.to_digit(10).unwrap(), 10).unwrap(),
        }
    }
}

fn char_heightmap_to_i32_height(heightmap: &Array2<char>) -> Array2<i32> {
    let i32_heightmap = heightmap.map(|x| match *x {
        'S' => START_HEIGHT,
        'E' => END_HEIGHT,
        val => val as i32 - 'a' as i32,
    });
    i32_heightmap
}

pub struct Day12part1 {
    file: PathBuf,
    raw_heightmap: Array2<char>,
    heightmap: Array2<i32>,
}

impl Day12part1 {
    pub fn new(input_path: PathBuf) -> Self {
        Self {
            file: input_path,
            raw_heightmap: Array2::from_elem([0, 0], ' '),
            heightmap: Array2::from_elem([0, 0], 0),
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
        self.raw_heightmap
            .indexed_iter()
            .map(|((y, x), c)| (Point::new(x, y), *c))
            .find_map(|(point, c)| {
                if c == find_char {
                    Some(self.get_point_data(point))
                } else {
                    None
                }
            })
    }

    fn path_find(&self, start: &Point, /*should_stop: impl Fn(&Point) -> bool*/) -> Vec<Vec<Point>> {
        let mut path = vec![self.get_point_data(*start)];
        let mut paths = Vec::new();
        loop {
            let current_data = path.last().unwrap();
            let neighbors = self.neighbors(current_data.point);
            match neighbors.iter().find(|data| data.height == END_HEIGHT) {
                None => {}
                Some(data) => {
                    path.push(*data)
                }
            }
            todo!();
        }
        paths
    }

    fn create_point(&self, x: i32, y: i32) -> Option<Point> {
        let point = match (usize::try_from(x), usize::try_from(y)) {
            (Ok(x), Ok(y)) => Point::new(x, y),
            _ => return None,
        };

        let (bounds_y, bounds_x) = self.heightmap.shape().map_to(|x| (x[0], x[1]));
        if point.x >= bounds_x || point.y >= bounds_y {
            return None;
        }
        Some(point)
    }

    fn get_point_data(&self, point: Point) -> PointData {
        PointData {
            point,
            height: self.heightmap[point.as_usize_array_yx()],
        }
    }

    fn neighbors(&self, point: Point) -> Vec<PointData> {
        let current_height = self.heightmap[point.as_usize_array_yx()];
        let mut neighbors = Vec::new();
        for offset in ALLOWED_OFFSETS.iter() {
            let [x, y] = point.map_to(|Point { x, y }| [x as i32 + offset.x, y as i32 + offset.y]);
            match self.create_point(x, y) {
                Some(point) => {
                    let height = self.heightmap[point.as_usize_array_yx()];
                    if (height - current_height).abs() > 1 {
                        continue;
                    }
                    neighbors.push(self.get_point_data(point));
                }
                None => continue,
            }
        }
        neighbors
    }

    pub fn solve(&mut self) {
        self.raw_heightmap = self.parse();
        self.heightmap = char_heightmap_to_i32_height(&self.raw_heightmap);
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
