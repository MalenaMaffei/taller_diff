//! Module that holds logic to parse the filenames
/// Abstraction that holds the provided file names
pub struct FileNamesParser {
    pub file_a: String,
    pub file_b: String,
}

impl FileNamesParser {
    /// parses a list args and returns a FileNamesParser holding the
    /// filenames provided as args
    pub fn new(args: &[String]) -> Result<FileNamesParser, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let file_a = args[1].clone();
        let file_b = args[2].clone();
        Ok(FileNamesParser { file_a, file_b })
    }
}
