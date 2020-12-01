const DATA_FILE: &str = "data/day_1.txt";
const TARGET: u32 = 2020;

pub fn solve_part_1(){
  println!("Solving day one - part one");
  match calculate_part_one() {
    Some(x) => println!("The answer is {}", x),
    None => println!("Can't find solution")
  }
}

pub fn solve_part_2(){
  println!("Solving day one - part two");
  match calculate_part_two() {
    Some(x) => println!("The answer is {}", x),
    None => println!("Can't find solution")
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
  let raw_data = crate::util::ioutil::read_file(DATA_FILE);
  let str_array = raw_data.split_whitespace();
  return str_array.into_iter().map(|s| s.parse::<u32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn assert_parsing_finds_values() {
    let values = prepare_data();
    assert!(!values.is_empty());
  }
  #[test]
  fn assert_values_are_summable() {
    let values: Vec<u32> = prepare_data();
    assert!(values.iter().sum::<u32>() > 0);
  }
  #[test]
  fn assert_part_one() {
    let result = calculate_part_one();
    assert!(!result.is_none());
    assert_eq!(result.unwrap(), 744475u32)
  }
  #[test]
  fn assert_part_two() {
    let result = calculate_part_two();
    assert!(!result.is_none());
    assert_eq!(result.unwrap(), 70276940u32)
  }
}