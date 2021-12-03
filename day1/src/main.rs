use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(&filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn get_input(filename: &str) -> Vec<i32> {
  let lines = read_lines(filename).expect("Somehow failed to open the file");
  let input = lines.map(|line| line.unwrap().parse().unwrap()).collect();
  input
}

fn compare_two(input: Vec<i32>) -> i32 {
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

fn compare_rolling_window(input: Vec<i32>) -> i32 {
  let sums = input
    .windows(3)
    .map(|triplet| triplet.iter().sum())
    .collect();

  compare_two(sums)
}

fn main() {
  println!("total: {}", compare_two(get_input("1-1-input.txt")));
  println!(
    "total: {}",
    compare_rolling_window(get_input("1-1-input.txt"))
  );
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_compare_two() {
    assert_eq!(compare_two(get_input("1-1-dummy.txt")), 7);
  }

  #[test]
  fn test_compare_rolling_window() {
    assert_eq!(compare_rolling_window(get_input("1-2-dummy.txt")), 5);
  }
}
