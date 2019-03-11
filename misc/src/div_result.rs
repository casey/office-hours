// enum Result<T, E> {
//   Ok(T),
//   Err(E),
// }

use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
enum Error {
  DivideByZero,
  Parse(ParseFloatError),
}

// needed for implicit into()
impl From<ParseFloatError> for Error {
  fn from(parse_float_error: ParseFloatError) -> Error {
    Error::Parse(parse_float_error)
  }
}

fn actual_div(numerator: f64, denominator: f64) -> f64 {
  numerator / denominator
}

fn div(numerator: &str, denominator: &str) -> Result<String, Error> {
  let numerator: f64 = numerator.parse()?;
  let denominator: f64 = denominator.parse()?;

  if denominator == 0.0 {
    Err(Error::DivideByZero)
  } else {
    Ok(actual_div(numerator, denominator).to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    assert_eq!(div("4", "2"), Ok("2".to_string()));
  }

  #[test]
  fn larger() {
    // utf 32
    // utf 16 // fuckin' sucks
    // utf  8 // ascii compatible, multibyte encoding

    let mut string_proper: String = String::new(); // like a Vec<u8>
    string_proper.push('4');

    let numerator: String = "8".to_string();
    let denominator: String = "2".to_string();

    // deref conversion: &String -> &str (for example)
    assert_eq!(div(&numerator, &denominator), Ok(string_proper));
  }

  #[test]
  fn zero() {
    assert_eq!(div("10", "0"), Err(Error::DivideByZero));
  }

  #[test]
  fn parse_error() {
    match div("XYZ", "0") {
      Err(Error::Parse(_)) => {}
      Err(err) => panic!("Expected parse error but got: {:?}", err),
      Ok(ok) => panic!("Expected error but got: {}", ok),
    }
  }
}
