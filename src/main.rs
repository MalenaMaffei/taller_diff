use std::env;
use std::process;
mod file;
mod filenamesparser;
use std::error::Error;
mod grid;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_names = filenamesparser::FileNamesParser::new(&args).unwrap_or_else(|err| {
        println!("Encountered a problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Generating diff for {}", file_names.file_a);
    println!("Against {}", file_names.file_b);

    if let Err(e) = run(file_names.file_a, file_names.file_b) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn run(filename_a: String, filename_b: String) -> Result<(), Box<dyn Error>> {
    let mut file_a = file::File::new(filename_a)?;
    let mut file_b = file::File::new(filename_b)?;

    let mut lcs_grid = grid::LCSGrid::new(file_a.len_lines, file_b.len_lines);

    lcs_grid.construct_grid(&mut file_a.contents, &mut file_b.contents);
    taller_diff::print_diff(
        &lcs_grid.grid,
        &file_a.contents,
        &file_b.contents,
        file_a.contents.len(),
        file_b.contents.len(),
    );
    Ok(())
}
