use std::fs;
use std::path::Path;

pub fn read_list_of_numbers<P: AsRef<Path>>(file: P, sep: &str) -> Vec<i32> {
    fs::read_to_string(file)
        .unwrap()
        .split(sep)
        .map(|item| item.parse::<i32>().unwrap())
        .collect()
}

pub fn read_list_of_strings<P: AsRef<Path>>(file: P, sep: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap()
        .split(sep)
        .map(String::from)
        .collect()
}
