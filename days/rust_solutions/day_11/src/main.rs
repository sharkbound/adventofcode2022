mod day_11_part_1;
mod day_11_part_2;

fn main() {
    let input_path = r"D:\git\adventofcode2022\inputs\day_11.txt";
    // let sample_0_path = r"D:\git\adventofcode2022\days\day_11\day_11\src\sample_0.txt";
    day_11_part_1::Day11Part1::new(input_path).solve();
    day_11_part_2::Day11Part2::new(input_path).solve();
}
