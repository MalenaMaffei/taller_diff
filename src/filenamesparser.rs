pub struct FileNamesParser {
    pub file1: String,
    pub file2: String,
}

impl FileNamesParser {
    pub fn new(args: &[String]) -> Result<FileNamesParser, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let file1 = args[1].clone();
        let file2 = args[2].clone();
        Ok(FileNamesParser { file1, file2 })
    }
}
