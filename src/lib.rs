use std::cmp;
use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub struct FileNames {
    pub file1: String,
    pub file2: String,
}

impl FileNames {
    pub fn new(args: &[String]) -> Result<FileNames, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let file1 = args[1].clone();
        let file2 = args[2].clone();
        Ok(FileNames { file1, file2 })
    }
}

// pub fn file_to_vec(filename : String) -> Result<Vec<String>, &'static str> {
//     // let opened_file = fs::File::open(filename)?;
//     let opened_file = match fs::File::open(filename){
//         Ok(opened_file) => opened_file,
//         Err(_) => panic!("File not found")
//     };
//     let file_reader = io::BufReader::new(opened_file);
//     println!()
//     Ok(file_reader.lines().filter_map(Result::ok).collect())
// }
pub fn file_to_vec(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    io::BufReader::new(fs::File::open(filename)?)
        .lines()
        .collect()
}

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

pub struct LCSGrid {
    pub grid: Vec<Vec<usize>>,
}

impl LCSGrid {
    pub fn new(file_a_len: usize, file_b_len: usize) -> LCSGrid {
        let grid = vec![vec![0; file_a_len + 1]; file_b_len + 1];
        println!("Matrix:\n{:?}", grid);
        LCSGrid { grid }
    }

    pub fn construct_grid(&mut self, lines_a: &mut [String], lines_b: &mut [String]) {
        for (i, line_a) in lines_a.iter_mut().enumerate() {
            for (j, line_b) in lines_b.iter_mut().enumerate() {
                if *line_a == *line_b {
                    // println!("Iguales encontradas {} == {}", *line_a , *line_b );
                    self.grid[j + 1][i + 1] = self.grid[j][i] + 1;
                } else {
                    self.grid[j + 1][i + 1] = cmp::max(self.grid[j + 1][i], self.grid[j][i + 1]);
                }
            }
        }
        println!("Matrix:\n{:?}", self.grid);
    }
}

pub fn run(files: FileNames) -> Result<(), Box<dyn Error>> {
    let mut file_a = File::new(files.file1)?;
    let mut file_b = File::new(files.file2)?;

    let mut lcs_grid = LCSGrid::new(file_a.len_lines, file_b.len_lines);

    lcs_grid.construct_grid(&mut file_a.contents, &mut file_b.contents);
    print_diff(
        &lcs_grid.grid,
        &file_a.contents,
        &file_b.contents,
        file_a.contents.len(),
        file_b.contents.len(),
    );
    Ok(())
}

pub fn print_diff(
    grid: &Vec<Vec<usize>>,
    lines_a: &Vec<String>,
    lines_b: &Vec<String>,
    i: usize,
    j: usize,
) {
    if i > 0 && j > 0 && lines_a[i - 1] == lines_b[j - 1] {
        print_diff(grid, lines_a, lines_b, i - 1, j - 1);
        println!(" {}", lines_a[i - 1]);
    } else if j > 0 && (i == 0 || grid[j - 1][i] >= grid[j][i - 1]) {
        print_diff(grid, lines_a, lines_b, i, j - 1);
        println!("> {}", lines_b[j - 1]);
    } else if i > 0 && (j == 0 || grid[j - 1][i] < grid[j][i - 1]) {
        print_diff(grid, lines_a, lines_b, i - 1, j);
        println!("< {}", lines_a[i - 1]);
    } else {
        println!();
    }
}
