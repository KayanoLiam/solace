use std::fs;
use std::path::Path;

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) {
    fs::write(path, content).expect("Failed to write file");
}
