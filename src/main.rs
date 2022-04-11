use std::env;
use std::process;
mod file;
mod filenamesparser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_names = filenamesparser::FileNamesParser::new(&args).unwrap_or_else(|err| {
        println!("Encountered a problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Generating diff for {}", file_names.file_a);
    println!("Against {}", file_names.file_b);

    if let Err(e) = taller_diff::run(file_names.file_a, file_names.file_b) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
