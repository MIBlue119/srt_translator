use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct SrtLoader {}

impl SrtLoader {
    // A constructor for the SrtLoader struct
    pub fn new() -> SrtLoader {
        SrtLoader {}
    }
    // A method to load the srt file and return a vector of strings
    pub fn load_srt_file(&self, file_path: String) -> Vec<String> {
        // Check if the file exists
        let path = Path::new(&file_path);
        if path.exists() {
            println!("File exists");
            println!("file_path: {:#?}", file_path);

            // Open the file
            let file = File::open(file_path).unwrap();
            // Read the file line by line and store each line in a vector
            let mut lines = Vec::new();
            for line in BufReader::new(file).lines() {
                lines.push(line.unwrap());
            }
            // Return the vector
            return lines;
        } else {
            println!("File does not exist");
        }
        // Return an empty vector if the file does not exist
        Vec::new()
    }
}
