
use crate::util::ioutil::{ parse_file, ParseError };
use crate::Challenge;

const DATA_FILE: &str = "data/day_3.txt";

pub struct Day2 {}

impl Challenge for Day2 {

  fn solve(&self) -> (&str, Option<String>, Option<String>) {
    let p1 = solve_part_1().to_string();
    let p2 = solve_part_2().to_string();
    return ("Day 3", Some(p1), Some(p2));
  }

}

#[derive(PartialEq, Debug)] 
enum Thing {
  EMPTY,
  TREE
}

type World = Vec<Vec<Thing>>;

pub fn solve_part_1() -> u32 {
  return traverse_and_count(&prepare_data(), 3, 1);
}

pub fn solve_part_2() -> u32 {
  let data = prepare_data();
  return traverse_and_count(&data, 1, 1) * 
    traverse_and_count(&data, 3, 1) *
    traverse_and_count(&data, 5, 1) *
    traverse_and_count(&data, 7, 1) *
    traverse_and_count(&data, 1, 2);
}

fn traverse_and_count(world: &World, r_steps: usize, d_steps: usize) -> u32 {
  let mut x = 0;
  let mut y = 0;
  let mut trees_encountered = 0;
  let width = world[0].len();
  while x < world.len() {
    match world[x][y] {
      Thing::TREE => trees_encountered = trees_encountered + 1,
      _ => ()
    }
    y = y + r_steps;
    x = x + d_steps;
    if y >= width {
      y = y - width;
    }
  }
  return trees_encountered;
}

fn parse_row(row: &str) -> Result<Vec<Thing>, ParseError> {
  row.chars().map(|c| match c {
    '.' => Ok(Thing::EMPTY),
    '#' => Ok(Thing::TREE),
    _ => Err( ParseError { invalid_data: c.to_string() })
  })
  .collect()
}

fn prepare_data() -> World {
  match parse_file(DATA_FILE, parse_row) {
    Ok(v) => v,
    Err(e) => panic!("Can't parse: {}", e.invalid_data)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::util::ioutil::{ parse };
  #[test]
  fn assert_parsing_finds_values() {
    let values = prepare_data();
    assert!(!values.is_empty());
    assert!(values.len() > 5);
  }
  #[test]
  fn assert_parse_row() {
    let values = parse_row("..#..#");
    assert!(values.is_ok());
    let ok_values = values.unwrap();
    assert_eq!(ok_values[0], Thing::EMPTY);
    assert_eq!(ok_values[1], Thing::EMPTY);
    assert_eq!(ok_values[2], Thing::TREE);
    assert_eq!(ok_values[3], Thing::EMPTY);
    assert_eq!(ok_values[4], Thing::EMPTY);
    assert_eq!(ok_values[5], Thing::TREE);
  }
  #[test]
  fn assert_parse_world_and_traverse() {
    let world = "####\n\
                 ####\n\
                 ####\n\
                 ####\n";

    let p_world = parse(world.to_string(), parse_row);
    assert!(p_world.is_ok());
    let count = traverse_and_count(&p_world.unwrap(), 1, 1);
    assert_eq!(count, 4);
  }
  #[test]
  fn assert_parse_world_and_traverse_with_rolling_world() {
    let world = "####\n\
                 ####\n\
                 ####\n\
                 ####\n\
                 ####\n\
                 ####\n\
                 ####\n\
                 ####\n";

    let p_world = parse(world.to_string(), parse_row);
    assert!(p_world.is_ok());
    let count = traverse_and_count(&p_world.unwrap(), 1, 1);
    assert_eq!(count, 8);
  }
  #[test]
  fn assert_part_one() {
    let c = solve_part_1();
    assert_eq!(c, 284);
  }

  #[test]
  fn assert_part_two() {
    let c = solve_part_2();
    assert_eq!(c, 3510149120);
  }
}