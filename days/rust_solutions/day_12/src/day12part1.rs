use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;
use std::path::PathBuf;

pub struct Day12part1 {
    file: File,
    heightmap: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone)]
struct CharPos {
    y: usize,
    x: usize,
    chr: char,
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
            .filter_map(|(row_idx, row)|
                match row.iter().position(|chr| *chr == find_char) {
                    Some(col_idx) => Some((row_idx, col_idx)),
                    None => None
                }
            ).nth(0).unwrap(); // todo: panic here
        CharPos {
            y: data.0,
            x: data.1,
            chr: find_char,
        }
    }

    fn _start_end(&self) -> (CharPos, CharPos) {
        (self._find_char('S'), self._find_char('E'))
    }

    pub fn solve(&mut self) {
        let heightmap = self.parse();
        let (start, end) = self._start_end();
        println!("{:?} {:?}", start, end);
    }
}