use std::error::Error;
mod file;
mod grid;

pub fn run(filename_a: String, filename_b: String) -> Result<(), Box<dyn Error>> {
    let mut file_a = file::File::new(filename_a)?;
    let mut file_b = file::File::new(filename_b)?;

    let mut lcs_grid = grid::LCSGrid::new(file_a.len_lines, file_b.len_lines);

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
