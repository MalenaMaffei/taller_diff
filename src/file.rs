//! Module that holds the file abstraction
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// File struct that holds its contents and its lenghth
pub struct File {
    pub contents: Vec<String>,
    pub len_lines: usize,
}

impl File {
    /// Returns a new instance of File, getting its contents and
    /// length from a filename
    pub fn new(filename: String) -> Result<File, &'static str> {
        let contents = file_to_vec(filename).expect("Could not load lines");
        let len_lines = contents.len();
        Ok(File {
            contents,
            len_lines,
        })
    }
}

/// Opens the provided filename and converts each line to an element in a
/// vector, returning said vecetor
pub fn file_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    io::BufReader::new(fs::File::open(filename)?)
        .lines()
        .collect()
}
