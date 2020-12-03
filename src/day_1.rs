use crate::Challenge;

const DATA_FILE: &str = "data/day_1.txt";
const TARGET: u32 = 2020;

pub struct Day1 {}

impl Challenge for Day1 {

  fn solve(&self) -> (&str, Option<String>, Option<String>) {
    let p1 = match calculate_part_one() {
      Some(res) => Some(res.to_string()),
      None => None
    };
    let p2 = match calculate_part_two() {
      Some(res) => Some(res.to_string()),
      None => None
    };
    return ("Day 1", p1, p2);
  }

}

fn calculate_part_one() -> Option<u32> {
  let numbers = prepare_data();
  for a in 0..numbers.len() {
    for b in a + 1..numbers.len() {
      if numbers[a] + numbers[b] == TARGET {
        return Some(numbers[a] * numbers[b])
      }
    } 
  }
  None
}

fn calculate_part_two() -> Option<u32> {
  let numbers = prepare_data();
  for a in 0..numbers.len() {
    for b in a + 1..numbers.len() {
      for c in b + 1..numbers.len() {
        if numbers[a] + numbers[b] + numbers[c] == TARGET {
          return Some(numbers[a] * numbers[b] * numbers[c])
        }
      }
    } 
  }
  None
}

fn prepare_data() -> Vec<u32> {
  match crate::util::ioutil::parse_file(DATA_FILE, |s| match s.parse::<u32>() {
    Ok(v) => Ok(v),
    Err(_) => Err(crate::util::ioutil::ParseError { invalid_data: s.to_string() })
  }) {
    Ok(data) => data,
    Err(e) => panic!("Erro while parsing: {}", e.invalid_data)
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
  fn assert_values_are_summable() {
    let values: Vec<u32> = prepare_data();
    assert!(values.iter().sum::<u32>() > 0);
  }
  #[test]
  fn assert_part_one_solution() {
    let result = calculate_part_one();
    assert!(!result.is_none());
    assert_eq!(result.unwrap(), 744475u32)
  }
  #[test]
  fn assert_part_two_solution() {
    let result = calculate_part_two();
    assert!(!result.is_none());
    assert_eq!(result.unwrap(), 70276940u32)
  }
}