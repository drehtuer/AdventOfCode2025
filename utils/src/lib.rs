use std::io;
use std::fs;

pub fn read(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn split(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
}
