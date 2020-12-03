use regex::Regex;
use lazy_static::lazy_static;
use crate::util::ioutil::{ parse_file, ParseError };

const DATA_FILE: &str = "data/day_2.txt";

#[derive(Debug)]
struct PwdEntry {
  min: u8,
  max: u8,
  character: char,
  pwd: String
}

pub fn solve_part_1(){
  println!("Solving day two - part one");
  println!("Found {} valid pwds", 
    valid_pwd_count(prepare_data(), valid_pwd_first_part))
}

pub fn solve_part_2(){
  println!("Solving day two - part two");
  println!("Found {} valid pwds", 
    valid_pwd_count(prepare_data(), valid_pwd_second_part))
}

fn valid_pwd_count(entries: Vec<PwdEntry>, validator: fn(&PwdEntry) -> bool) -> usize {
  return entries
    .into_iter()
    .filter(validator)
    .collect::<Vec<PwdEntry>>()
    .len();
}

fn prepare_data() -> Vec<PwdEntry> {
  match parse_file(DATA_FILE, parse_pwd) {
    Ok(v) => v,
    Err(r) => panic!("Can't parse: {}", r.invalid_data)
  }
}

fn valid_pwd_first_part(entry: &PwdEntry) -> bool {
  let mut found = 0;
  for c in entry.pwd.chars() {
    if c == entry.character {
      found = found + 1;
    }
  }
  return found >= entry.min && found <= entry.max;
}

fn valid_pwd_second_part(entry: &PwdEntry) -> bool {
  // Ok, calling nth() consumes preceeding chars..
  // There must be a better way...
  let mut chars = entry.pwd.chars();
  let first = chars.nth((entry.min - 1).into()).unwrap_or('_');
  let second = chars.nth(((entry.max - 1) - (entry.min - 1) - 1).into()).unwrap_or('_');
  
  return (first == entry.character || second == entry.character) && first != second;
}

fn parse_pwd(raw_line: &str) -> Result<PwdEntry, ParseError> {
  lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\S): (.+)").unwrap();
  }
  let matches: Option<regex::Captures> = RE.captures(raw_line);
  match matches {
    Some(captures) => Ok(PwdEntry {
      min: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
      max: captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
      character: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
      pwd: captures.get(4).unwrap().as_str().to_string(),
    }),
    None => {
      println!("Can't parse {}", raw_line);
      return Err( ParseError { invalid_data: raw_line.to_string() });
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn assert_parsing_finds_values() {
    let values = prepare_data();
    assert!(!values.is_empty());
    assert!(values.len() > 5);
  }
  #[test]
  fn assert_parse_fail() {
    let entry = parse_pwd("vvevervv");
    assert!(entry.is_err());
    assert_eq!(entry.unwrap_err().invalid_data, "vvevervv")
  }
  #[test]
  fn assert_parse_entry() {
    let entry = parse_pwd("1-3 c: abcdefg");
    assert!(entry.is_ok());
    let uw_entry = entry.unwrap();
    assert_eq!(uw_entry.min, 1u8);
    assert_eq!(uw_entry.max, 3u8);
    assert_eq!(uw_entry.character, 'c');
    assert_eq!(uw_entry.pwd, "abcdefg");
  }
  #[test]
  fn assert_parse_entry_multiple_characters_in_digits() {
    let entry = parse_pwd("13-53 c: abcdefg");
    assert!(entry.is_ok());
    let uw_entry = entry.unwrap();
    assert_eq!(uw_entry.min, 13u8);
    assert_eq!(uw_entry.max, 53u8);
    assert_eq!(uw_entry.character, 'c');
    assert_eq!(uw_entry.pwd, "abcdefg");
  }
  #[test]
  fn assert_valid_pwd_first_part() {
    assert!(valid_pwd_first_part(&parse_pwd("1-1 a: a").unwrap()));
    assert!(valid_pwd_first_part(&parse_pwd("1-2 a: aa").unwrap()));
    assert!(!valid_pwd_first_part(&parse_pwd("1-2 a: b").unwrap()));
  }
  #[test]
  fn assert_valid_pwd_second_part() {
    assert!(!valid_pwd_second_part(&parse_pwd("1-2 a: aa").unwrap()));
    assert!(valid_pwd_second_part(&parse_pwd("1-3 c: cab").unwrap()));
    assert!(valid_pwd_second_part(&parse_pwd("1-3 c: bac").unwrap()));
    assert!(!valid_pwd_second_part(&parse_pwd("1-2 a: bcd").unwrap()));
  }
  #[test]
  fn assert_valid_pwd_counter_adds() {
    let entries = vec![ 
      PwdEntry { min: 1, max: 2, character: 'a', pwd: "xxxx".to_string()} 
    ];
    assert_eq!(valid_pwd_count(entries, |_| true), 1);
    // Borrow checker...
    // assert_eq!(valid_pwd_count(entries, |_| false), 0);
  }
  #[test]
  fn assert_valid_pwd_counter_drops() {
    let entries = vec![ 
      PwdEntry { min: 1, max: 2, character: 'a', pwd: "xxxx".to_string()} 
    ];
    assert_eq!(valid_pwd_count(entries, |_| false), 0);
  }
  #[test]
  fn assert_correct_answer_part_1() {
    assert_eq!(valid_pwd_count(prepare_data(), valid_pwd_first_part), 607);
  }
  #[test]
  fn assert_correct_answer_part_2() {
    assert_eq!(valid_pwd_count(prepare_data(), valid_pwd_second_part), 321);
  }
}