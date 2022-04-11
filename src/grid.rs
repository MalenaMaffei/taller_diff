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
}
