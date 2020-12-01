use std::fs;

pub fn read_file(filename: &str) -> String {
  return fs::read_to_string(filename).expect("Can't open file");
}

