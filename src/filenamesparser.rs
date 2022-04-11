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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_filename_correct() {
        let filenames = FileNamesParser::new(&["0".to_string(), "a".to_string(), "b".to_string()]).expect("not enough args");
        println!("{}",filenames.file_a);
        assert_eq!(filenames.file_a, "a".to_string());
    }

    #[test]
    fn second_filename_correct() {
        let filenames = FileNamesParser::new(&["0".to_string(), "a".to_string(), "b".to_string()]).expect("not enough args");
        assert_eq!(filenames.file_b, "b".to_string());
    }

    #[test]
    fn ignores_other_args() {
        let filenames = FileNamesParser::new(&["0".to_string(), "a".to_string(), "b".to_string(), "ignore".to_string()]).expect("not enough args");
        assert_eq!(filenames.file_b, "b".to_string());
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn not_enough_args() {
        FileNamesParser::new(&["a".to_string(), "b".to_string()]).expect("not enough arguments");
    }
}