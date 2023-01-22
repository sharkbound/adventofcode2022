use rustc_hash::FxHashSet;
use rustutils::apply_ext::ApplyOwnExt;
use rustutils::collections::CollectToVec;
use rustutils::io::FileBuilder;
use rustutils::iterable_string_ext::JoinToStringExt;
use rustutils::logging::DebugLog;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Eq, Hash)]
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

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
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
    fn height_at_point(&self, point: &Point) -> Option<u32>;
    fn valid_moves(&self, point: &Point) -> Vec<(Point, u32)>;
    fn neighbors(&self, point: &Point) -> Vec<Point>;
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

    fn height_at_point(&self, point: &Point) -> Option<u32> {
        self.height(point.0, point.1)
    }

    fn valid_moves(&self, point: &Point) -> Vec<(Point, u32)> {
        let current_height = match self.height_at_point(point) {
            None => {
                return vec![];
            }
            Some(x) => x,
        };

        self.neighbors(point)
            .iter()
            .filter_map(|p| match self.height_at_point(p) {
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

    fn neighbors(&self, point: &Point) -> Vec<Point> {
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

pub struct Day12part1 {
    file: PathBuf,
    heightmap: Vec<Vec<char>>,
}

impl Day12part1 {
    pub fn new(input_path: PathBuf) -> Self {
        Self {
            file: input_path,
            heightmap: Vec::new(),
        }
    }

    pub fn parse(&self) -> Vec<Vec<char>> {
        match FileBuilder::from_pathbuf(self.file.clone())
            .read()
            .buffered_reader()
        {
            Ok(reader) => reader,
            Err(e) => panic!("Cannot read {:?}: {}", self.file, e),
        }
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

    fn path_find<F: Fn(&Point) -> bool>(&self, start: &Point, should_stop: F) -> Vec<Vec<Point>> {
        let mut path_queue: Vec<(Point, Vec<Point>, FxHashSet<Point>)> = Vec::new();
        let mut paths = Vec::<Vec<Point>>::new();
        path_queue.push((*start, vec![], FxHashSet::default()));

        while !path_queue.is_empty() {
            let (node, mut path, mut seen) = path_queue.pop().unwrap();

            let valid_moves = self
                .heightmap
                .valid_moves(&node)
                .into_iter()
                .filter(|(cur, _)| *cur != node)
                .collect_to_vec();

            if valid_moves.is_empty() {
                continue;
            }

            for (point, _) in valid_moves {
                path.push(point);

                if should_stop(&point) {
                    println!(
                        "path terminated: {:?}",
                        path.iter()
                            .join_to_string("->", |x| format!("({},{})", x.0, x.1))
                    );
                    paths.push(path.iter().map(|x| *x).collect_to_vec());
                    continue;
                }

                if seen.contains(&point) {
                    continue;
                }

                seen.insert(point);
                // path.last().debug();
                path_queue.push((
                    point,
                    {
                        let mut v = Vec::new();
                        path.clone_into(&mut v);
                        v.push(point);
                        v
                    },
                    seen.clone(),
                ));
            }
        }
        paths
    }

    pub fn solve(&mut self) {
        self.heightmap = self.parse();
        let (start, end) = (self._find_char('S'), self._find_char('E'));
        let mut paths = self
            .path_find(&start.point, |p| p.1 == self.heightmap[0].len() / 2)
            .iter()
            .map(|x| x.clone())
            .collect_to_vec();
        paths.sort_by_key(|x| x.len());
        println!("{:?}", paths.first().unwrap().len());
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