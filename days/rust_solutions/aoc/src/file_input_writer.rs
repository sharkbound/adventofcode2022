use std::path::{PathBuf};


pub fn input_directory_path() -> std::io::Result<PathBuf> {
    PathBuf::from("../../inputs/").canonicalize()
}

pub fn input_file_path(day_number: u32) -> Result<PathBuf, std::io::Error> {
    let mut path = input_directory_path()?;
    path.push(format!("day_{}.txt", day_number));
    Ok(path)
}

