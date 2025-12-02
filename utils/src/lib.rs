use std::fs;
use std::io;

pub fn read(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn split(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
}

pub fn number_of_digits(number: u64) -> u32 {
    if number == 0 {
        return 1;
    }
    ((number as f64).log10().floor() as u32) + 1
}
