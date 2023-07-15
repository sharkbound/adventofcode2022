use rustutils::path_from_root;

#[allow(unused_variables)]
fn main() {
    let root = std::env::current_dir().unwrap().canonicalize().unwrap();

    let input = path_from_root!(root, "inputs", "day_12.txt");
    let sample = path_from_root!(root, "day_12", "src", "sample_0.txt");

    day_12::day12part1::Day12part1::new(sample).solve();
}
