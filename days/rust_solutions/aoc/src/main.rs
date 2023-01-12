use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use aoc::get_aoc_input;
use rustutils::arg_parser::args::{ArgCollection, ArgDataType};

#[tokio::main]
async fn main() {
    let mut parser = ArgCollection::new();
    parser.add_required("day", None, "day of the input to download", ArgDataType::I32);
    parser.add_optional("out", None, "file to write the input text to", ArgDataType::String, "");
    parser.bind_env_args();
    parser.check();

    let day = parser.parse_u32("day").unwrap();
    let input = get_aoc_input(day).await;

    let input_text = match input {
        Ok(value) => { value }
        Err(err) => {
            println!("Cannot download AOC input for day: {}\nErr: \n\t{}", day, err);
            std::process::exit(1);
        }
    };

    let output_path = match parser.parse_string("out") {
        Ok(ref path) => match path.as_str() {
            "" => input_file_path(day),
            path => PathBuf::from(path),
        }
        Err(_) => input_file_path(day)
    };

    println!("Got AOC day {} input!", day);
    println!("Writing day {} input to file: {:?}", day, output_path);

    let mut file = BufWriter::new(File::create(output_path.clone()).unwrap());
    match file.write(input_text.as_bytes()) {
        Ok(_) => {
            println!("DONE! Wrote input to path: {:?}", output_path);
        }
        Err(err) => {
            println!("ERROR! Could not write input to file: {:?}", err);
            std::process::exit(1);
        }
    }
}

pub fn input_directory_path() -> PathBuf {
    PathBuf::from(r"D:\git\adventofcode2022\inputs")
}

pub fn input_file_path(day_number: u32) -> PathBuf {
    let mut path = input_directory_path();
    path.push(format!("day_{}.txt", day_number));
    path
}