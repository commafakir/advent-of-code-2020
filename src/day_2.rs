use regex::Regex;
use lazy_static::lazy_static;

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
  let count = valid_count_for_one(prepare_data());
  println!("Found {} valid pwds", count)
}

pub fn solve_part_2(){
  println!("Solving day two - part two");
  let count = valid_count_for_two(prepare_data());
  println!("Found {} valid pwds", count)
}

fn valid_count_for_one(entries: Vec<String>) -> usize {
  let a: Vec<PwdEntry> = entries
    .into_iter()
    .map(|r| parse_pwd(&r).unwrap())
    .filter(|e| is_valid_part_one(e))
    .collect();
  return a.len();
}

fn valid_count_for_two(entries: Vec<String>) -> usize {
  let a: Vec<PwdEntry> = entries
    .into_iter()
    .map(|r| parse_pwd(&r).unwrap())
    .filter(|e| is_valid_part_two(e))
    .collect();
  return a.len();
}

fn prepare_data() -> Vec<String> {
  let raw_data = crate::util::ioutil::read_file(DATA_FILE);
  let str_array = raw_data.lines();
  return str_array.map(|s| s.to_string()).collect()
}

fn is_valid_part_one(entry: &PwdEntry) -> bool {
  let mut found = 0;
  for c in entry.pwd.chars() {
    if c == entry.character {
      found = found + 1;
    }
  }
  return found >= entry.min && found <= entry.max;
}

fn is_valid_part_two(entry: &PwdEntry) -> bool {
  // Ok, calling nth() consumes preceeding chars..
  let mut chars = entry.pwd.chars();
  let first = chars.nth((entry.min - 1).into()).unwrap_or('_');
  let second = chars.nth(((entry.max - 1) - (entry.min - 1) - 1).into()).unwrap_or('_');

  println!("{}, {}, {}", entry.pwd, first, second);
  
  return (first == entry.character || second == entry.character) && first != second;
}

fn parse_pwd(raw_line: &str) -> Option<PwdEntry> {
  lazy_static! {
    static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\S): (.+)").unwrap();
  }
  let matches: Option<regex::Captures> = RE.captures(raw_line);
  match matches {
    Some(captures) => Some(PwdEntry {
      min: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
      max: captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
      character: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
      pwd: captures.get(4).unwrap().as_str().to_string(),
    }),
    None => {
      println!("Can't parse {}", raw_line);
      return None;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn assert_parsing_finds_values() {
    let values = prepare_data();
    println!("{:?}", values);
    assert!(!values.is_empty());
    assert!(values.len() > 5);
  }
  #[test]
  fn assert_parse_entry() {
    let entry = parse_pwd("1-3 c: abcdefg");
    assert!(!entry.is_none());
    let uw_entry = entry.unwrap();
    assert_eq!(uw_entry.min, 1u8);
    assert_eq!(uw_entry.max, 3u8);
    assert_eq!(uw_entry.character, 'c');
    assert_eq!(uw_entry.pwd, "abcdefg");
  }
  #[test]
  fn assert_parse_entry_multiple_characters() {
    let entry = parse_pwd("13-53 c: abcdefg");
    assert!(!entry.is_none());
    let uw_entry = entry.unwrap();
    assert_eq!(uw_entry.min, 13u8);
    assert_eq!(uw_entry.max, 53u8);
    assert_eq!(uw_entry.character, 'c');
    assert_eq!(uw_entry.pwd, "abcdefg");
  }
  #[test]
  fn assert_is_valid_entry_part_one() {
    assert!(is_valid_part_one(&parse_pwd("1-1 a: a").unwrap()));
    assert!(is_valid_part_one(&parse_pwd("1-2 a: aa").unwrap()));
    assert!(!is_valid_part_one(&parse_pwd("1-2 a: b").unwrap()));
  }
  #[test]
  fn assert_is_valid_entry_part_two() {
    assert!(!is_valid_part_two(&parse_pwd("1-2 a: aa").unwrap()));
    assert!(is_valid_part_two(&parse_pwd("1-3 c: cab").unwrap()));
    assert!(is_valid_part_two(&parse_pwd("1-3 c: bac").unwrap()));
    assert!(!is_valid_part_two(&parse_pwd("1-2 a: bcd").unwrap()));
  }
  #[test]
  fn assert_valid_count_part_one() {
    assert_eq!(valid_count_for_one(vec![]), 0);
    assert_eq!(valid_count_for_one(vec!["1-1 a: a".to_string()]), 1);
    assert_eq!(valid_count_for_one(vec!["2-2 a: a".to_string()]), 0);
  }
  #[test]
  fn assert_valid_count_part_two() {
    assert_eq!(valid_count_for_two(vec![]), 0);
    assert_eq!(valid_count_for_two(vec!["1-3 a: avs".to_string()]), 1);
    assert_eq!(valid_count_for_two(vec!["2-3 a: afvafva".to_string()]), 0);
  }
}