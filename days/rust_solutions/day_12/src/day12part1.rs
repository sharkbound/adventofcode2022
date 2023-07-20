use ndarray::Array2;
use rustc_hash::FxHashSet;
use rustutils::logging::DebugLog;
use rustutils::map_to::MapToExt;
use std::net::UdpSocket;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct PointSubDiff {
    x: i32,
    y: i32,
    total: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn as_yx_array(&self) -> [usize; 2] {
        [self.y, self.x]
    }

    fn as_point_data(&self, heightmap: &Array2<i32>) -> PointData {
        PointData {
            point: *self,
            height: heightmap[self.as_yx_array()],
        }
    }

    fn sub_diff(&self, other: impl Into<Point>) -> PointSubDiff {
        let other = other.into();
        PointSubDiff {
            x: self.x as i32 - other.x as i32,
            y: self.y as i32 - other.y as i32,
            total: (self.x as i32 - other.x as i32).abs() + (self.y as i32 - other.y as i32).abs(),
        }
    }
}

impl Into<Point> for [usize; 2] {
    fn into(self) -> Point {
        Point {
            x: self[0],
            y: self[1],
        }
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

static START_HEIGHT: i32 = 0;
static END_HEIGHT: i32 = 27;
static START_CHAR: char = 'S';
static END_CHAR: char = 'E';

#[derive(Debug, Copy, Clone)]
struct PointData {
    pub point: Point,
    pub height: i32,
}

impl Into<Point> for PointData {
    fn into(self) -> Point {
        self.point
    }
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
    let i32_heightmap = heightmap.map(|&x| match x {
        'S' => START_HEIGHT,
        'E' => END_HEIGHT,
        val => val as i32 - 'a' as i32 + 1,
    });
    i32_heightmap
}

pub struct Day12part1 {
    data: &'static str,
    raw_heightmap: Array2<char>,
    heightmap: Array2<i32>,
}

impl Day12part1 {
    pub fn new(data: &'static str) -> Self {
        Self {
            data,
            raw_heightmap: Array2::from_elem([0, 0], ' '),
            heightmap: Array2::from_elem([0, 0], 0),
        }
    }

    pub fn parse(&self) -> Array2<char> {
        let lines = self.data.lines().collect::<Vec<_>>();

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
                    Some(point.as_point_data(&self.heightmap))
                } else {
                    None
                }
            })
    }

    fn find_best_path(&self, start: impl Into<Point>, target: impl Into<Point>) -> Vec<Point> {
        let start = start.into();
        let target = target.into();

        let mut path = vec![start];
        let mut best_path: Option<Vec<Point>> = None;
        let mut fully_explored = FxHashSet::default();

        while !path.is_empty() {
            let current_point = *path.last().unwrap();
            let current_point_data = current_point.as_point_data(&self.heightmap);

            if fully_explored.contains(&current_point) {
                path.pop();
                continue;
            }

            if current_point_data.height == END_HEIGHT {
                if path.len() < best_path.as_ref().map(|x| x.len()).unwrap_or(usize::MAX) {
                    best_path = Some(path.clone());
                }

                path.pop();
                match path.last() {
                    Some(p) => {
                        fully_explored.insert(*p);
                    }
                    _ => {}
                }
                continue;
            }

            let neighbors = self
                .neighbors(current_point)
                .into_iter()
                .filter(|PointData { point, .. }| {
                    !fully_explored.contains(point) && !path.contains(point)
                })
                .collect::<Vec<_>>();

            if neighbors.is_empty() {
                fully_explored.insert(current_point);
                path.pop();
                continue;
            }

            neighbors.debug();
            path.push(neighbors.first().unwrap().point);
        }

        best_path.unwrap_or(Vec::new())
    }

    fn create_point(&self, x: i32, y: i32) -> Option<Point> {
        let point = match (usize::try_from(x), usize::try_from(y)) {
            (Ok(x), Ok(y)) => Point { x, y },
            _ => return None,
        };

        let (bounds_y, bounds_x) = self.heightmap.shape().map_to(|x| (x[0], x[1]));
        if point.x >= bounds_x || point.y >= bounds_y {
            return None;
        }
        Some(point)
    }

    fn neighbors(&self, into_point: impl Into<Point>) -> Vec<PointData> {
        let center = into_point.into();
        let center_data = center.as_point_data(&self.heightmap);
        let current_height = self.heightmap[center.as_yx_array()];
        let mut neighbors = Vec::with_capacity(4);
        for offset in ALLOWED_OFFSETS.iter() {
            let [x, y] = center.map_to(|Point { x, y }| [x as i32 + offset.x, y as i32 + offset.y]);
            match self.create_point(x, y) {
                Some(point) => {
                    let data = point.as_point_data(&self.heightmap);
                    if data.height == END_HEIGHT
                        || (-1..=1).contains(&(center_data.height - data.height))
                    {
                        neighbors.push(data);
                    }
                }
                None => continue,
            }
        }
        neighbors
    }

    pub fn solve(&mut self) {
        self.raw_heightmap = self.parse();
        self.heightmap = char_heightmap_to_i32_height(&self.raw_heightmap);
        dbg!(&self.heightmap);
        let (start, end) = (self._find_char('S').unwrap(), self._find_char('E').unwrap());
        let path = self.find_best_path(start, end);
        // dbg!(&path);
        println!(
            "start: {:?}, end: {:?}\npath len: {:?}",
            start,
            end,
            path.len().checked_sub(1)
        );
    }
}

/*
[
    [-1, 0, 1, 16, 15, 14, 13, 12],
    [ 0, 1, 2, 17, 24, 23, 23, 11],
    [ 0, 2, 2, 18, 25, -2, 23, 10],
    [ 0, 2, 2, 19, 20, 21, 22,  9],
    [ 0, 1, 3,  4,  5,  6,  7,  8]
]
*/

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
