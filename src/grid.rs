//! Module that holds LCS Grid and the implementation of the LCS algorithm
use std::cmp;
/// Struct that represents the grid needed for LCS
pub struct LCSGrid {
    pub grid: Vec<Vec<usize>>,
}

impl LCSGrid {
    pub fn new(file_a_len: usize, file_b_len: usize) -> LCSGrid {
        let grid = vec![vec![0; file_a_len + 1]; file_b_len + 1];
        LCSGrid { grid }
    }

    pub fn construct_grid(&mut self, lines_a: &mut [String], lines_b: &mut [String]) {
        for (i, line_a) in lines_a.iter_mut().enumerate() {
            for (j, line_b) in lines_b.iter_mut().enumerate() {
                if *line_a == *line_b {
                    self.grid[j + 1][i + 1] = self.grid[j][i] + 1;
                } else {
                    self.grid[j + 1][i + 1] = cmp::max(self.grid[j + 1][i], self.grid[j][i + 1]);
                }
            }
        }
    }

    pub fn print_diff(&self, lines_a: &Vec<String>, lines_b: &Vec<String>, i: usize, j: usize) {
        if i > 0 && j > 0 && lines_a[i - 1] == lines_b[j - 1] {
            self.print_diff(lines_a, lines_b, i - 1, j - 1);
            println!(" {}", lines_a[i - 1]);
        } else if j > 0 && (i == 0 || self.grid[j - 1][i] >= self.grid[j][i - 1]) {
            self.print_diff(lines_a, lines_b, i, j - 1);
            println!("> {}", lines_b[j - 1]);
        } else if i > 0 && (j == 0 || self.grid[j - 1][i] < self.grid[j][i - 1]) {
            self.print_diff(lines_a, lines_b, i - 1, j);
            println!("< {}", lines_a[i - 1]);
        } else {
            println!();
        }
    }
}
