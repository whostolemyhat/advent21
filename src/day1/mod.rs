pub fn compare_two(input: Vec<i32>) -> i32 {
  // cool foldy solution nicked from github.com/desbo/advent-of-code-2021
  input.windows(2).fold(
    0,
    |total, pair| {
      if pair[1] > pair[0] {
        total + 1
      } else {
        total
      }
    },
  )
}

pub fn compare_rolling_window(input: Vec<i32>) -> i32 {
  let sums = input
    .windows(3)
    .map(|triplet| triplet.iter().sum())
    .collect();

  compare_two(sums)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::get_input;

  #[test]
  fn test_compare_two() {
    assert_eq!(compare_two(get_input("1-1-dummy.txt")), 7);
  }

  #[test]
  fn test_compare_rolling_window() {
    assert_eq!(compare_rolling_window(get_input("1-2-dummy.txt")), 5);
  }
}
