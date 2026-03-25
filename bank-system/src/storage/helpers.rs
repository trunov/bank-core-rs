use std::fs::File;
use std::io;

pub fn read_file(path: &str) -> io::Result<File> {
    let file = File::open(path)?;
    Ok(file)
}
