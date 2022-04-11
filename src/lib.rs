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
