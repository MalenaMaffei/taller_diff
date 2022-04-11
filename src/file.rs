use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub struct File {
    pub contents: Vec<String>,
    pub len_lines: usize,
}

impl File {
    pub fn new(filename: String) -> Result<File, &'static str> {
        let contents = file_to_vec(filename).expect("Could not load lines");
        let len_lines = contents.len();
        Ok(File {
            contents,
            len_lines,
        })
    }
}

pub fn file_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    io::BufReader::new(fs::File::open(filename)?)
        .lines()
        .collect()
}
