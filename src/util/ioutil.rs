use std::fs;

/// ParseError represents error occured while parsing single data entry
#[derive(Debug, Clone)]
pub struct ParseError {
  pub invalid_data: String
}

pub fn read_file(filename: &str) -> String {
  return fs::read_to_string(filename).expect("Can't open file");
}

/// Parses data from filename with given parser
pub fn parse_file<T>(filename: &str, parser: fn(&str) -> Result<T, ParseError>) -> Result<Vec<T>, ParseError> {
  return parse(read_file(filename), parser);
}

fn parse<T>(data: String, parser: fn(&str) -> Result<T, ParseError>) -> Result<Vec<T>, ParseError> {
  return data.lines().map(|raw| match parser(raw) {
    Ok(v) => Ok(v),
    Err(e) => Err(e)
  }).collect();

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lines_as_string() {
    let data = "aa\n\
                be\n\
                ce";
    let result = parse(data.to_string(), |x| Ok(x.to_string())).unwrap();
    assert_eq!(result[0], "aa");
    assert_eq!(result[1], "be");
    assert_eq!(result[2], "ce");
  }
  #[test]
  fn parse_numbers() {
    let data = "1\n\
                2\n\
                3";
    let result = parse(data.to_string(), |s| Ok(s.parse::<u32>().unwrap())).unwrap();
    assert_eq!(result[0], 1);
    assert_eq!(result[1], 2);
    assert_eq!(result[2], 3);
  }
  #[test]
  fn test_error() {
    let data = "1\n\
                abba\n\
                3";
    let result = parse(data.to_string(), 
      |s| match s.parse::<u32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(ParseError { invalid_data: s.to_string() })
      });
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().invalid_data, "abba");
  }
}