use std::env;
use std::process;
use taller_diff::FileNames;

fn main() {
    let args: Vec<String> = env::args().collect();

    let files = FileNames::new(&args).unwrap_or_else(|err| {
        println!("Encountered a problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Generating diff for {}", files.file1);
    println!("Against {}", files.file2);

    // println!("{}", taller_diff::file_to_vec(files.file1));
    // let lines = taller_diff::file_to_vec( files.file1)?;
    // println!("{:?}", lines);
    // let lines = taller_diff::file_to_vec(files.file1)
    // .expect("Could not load lines");
    // for line in lines {
    //     println!("{:?}", line);
    // }

    if let Err(e) = taller_diff::run(files) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
