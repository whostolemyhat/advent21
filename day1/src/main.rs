use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_higher_than_prev(previous: i32, next: i32) -> bool {
  next > previous
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(&filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn compare_two(filename: &str) -> i32 {
  let mut prev;
  let mut next = 0;
  let mut total = 0;
  let mut first_run = true;

  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(line) = line {
        prev = next;
        next = line.parse::<i32>().expect("Failed to parse line");

        if is_higher_than_prev(prev, next) && !first_run {
          total += 1;
        }
        first_run = false;
      }
    }
  }

  total
}

fn compare_rolling_window(filename: &str) -> i32 {
  let mut total = 0;
  let mut prev;
  let mut next = 0;
  let mut window: Vec<i32> = vec![];

  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(line) = line {
        let current = line.parse::<i32>().expect("Failed to parse line");
        if window.len() < 3 {
          window.push(current);
        } else {
          window.remove(0);
          window.push(current);
          prev = next;

          let window_total: i32 = window.iter().sum();
          next = window_total;
          if is_higher_than_prev(prev, next) {
            total += 1;
          }
        }
      }
    }
  }

  total
}

fn main() {
  println!("total: {}", compare_two("1-1-input.txt"));
  println!("total: {}", compare_rolling_window("1-1-input.txt"));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_higher_than_prev() {
    assert_eq!(is_higher_than_prev(10, 19), true);
    assert_eq!(is_higher_than_prev(20, 19), false);
    assert_eq!(is_higher_than_prev(20, 20), false);
  }

  #[test]
  fn test_compare_two() {
    assert_eq!(compare_two("1-1-dummy.txt"), 7);
  }

  #[test]
  fn test_compare_rolling_window() {
    assert_eq!(compare_rolling_window("1-2-dummy.txt"), 5);
  }
}
