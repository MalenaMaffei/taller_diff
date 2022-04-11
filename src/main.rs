use std::env;
use std::process;
mod file;
mod filenamesparser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let files = filenamesparser::FileNamesParser::new(&args).unwrap_or_else(|err| {
        println!("Encountered a problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Generating diff for {}", files.file1);
    println!("Against {}", files.file2);

    if let Err(e) = taller_diff::run(files.file1, files.file2) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
