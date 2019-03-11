fn div(numerator: u64, denominator: u64) -> Option<u64> {
  if denominator == 0 {
    None
  } else {
    Some(numerator / denominator)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    assert_eq!(div(4, 2), Some(2));
  }

  #[test]
  fn larger() {
    assert_eq!(div(8, 2), Some(4));
  }

  #[test]
  fn zero() {
    assert_eq!(div(10, 0), None);
  }
}
