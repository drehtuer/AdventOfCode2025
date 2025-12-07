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

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![value; rows * cols],
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> () {
        if row >= self.rows || col >= self.cols {
            return;
        }
        self.data[row * self.cols + col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(&self.data[row * self.cols + col])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(&mut self.data[row * self.cols + col])
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}
