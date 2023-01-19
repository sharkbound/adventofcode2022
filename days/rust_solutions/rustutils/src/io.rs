use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, ErrorKind};
use std::path::PathBuf;

pub struct FileBuilder {
    path: PathBuf,
    options: OpenOptions,
}

impl FileBuilder {
    pub fn new(path: PathBuf) -> FileBuilder {
        FileBuilder {
            path,
            options: OpenOptions::new(),
        }
    }

    pub fn read(&mut self) -> &mut Self {
        self.options.read(true);
        self
    }

    pub fn write(&mut self) -> &mut Self {
        self.options.write(true);
        self
    }

    pub fn append(&mut self) -> &mut Self {
        self.options.append(true);
        self
    }

    pub fn create(&mut self) -> &mut Self {
        self.options.create(true);
        self
    }

    pub fn create_new(&mut self) -> &mut Self {
        self.options.create_new(true);
        self
    }

    pub fn buffered_reader(&self) -> Result<BufReader<File>, Box<dyn Error>> {
        Ok(BufReader::new(
            self.options.open(self.path.clone()).map_err(Box::new)?,
        ))
    }

    pub fn file(&self) -> Result<File, Box<std::io::Error>> {
        self.options.open(self.path.clone()).map_err(Box::new)
    }
}
