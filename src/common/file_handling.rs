use std::fs::File;
use std::path::{Path, PathBuf};

use std::io::{self};
use std::io::Lines;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}

pub fn write_string_to_file(filename: &str, string_buffer: &String) {

    // 1. Create or overwrite the file
    let file = File::create(filename).expect("Creating file failed!");

    // 2. Wrap the file in a BufWriter
    let mut writer = BufWriter::new(file);

    // 3. Write data
    write!(writer, "{}", string_buffer);

    // 4. Explicitly flush the remaining data to disk
    writer.flush().expect("flush failed!");
}