use std::path::PathBuf;

fn main() {
    let sample = ["day_12", "src", "sample_0.txt"].iter().collect::<PathBuf>();
    let input = ["../../", "inputs", "day_12.txt"].iter().collect::<PathBuf>();
    day_12::day12part1::Day12part1::new(sample).solve();
}
