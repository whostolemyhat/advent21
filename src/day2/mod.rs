use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
  Forward,
  Down,
  Up,
}

impl FromStr for Direction {
  type Err = ();
  fn from_str(input: &str) -> Result<Direction, Self::Err> {
    match input {
      "forward" => Ok(Direction::Forward),
      "down" => Ok(Direction::Down),
      "up" => Ok(Direction::Up),
      _ => Err(()),
    }
  }
}

pub fn calc_position(input: Vec<String>) -> i32 {
  let mut horz_pos = 0;
  let mut vert_pos = 0;

  for cmd in input {
    let (direction, amount) = format_command(cmd);

    match direction {
      Direction::Forward => horz_pos += amount,
      Direction::Down => vert_pos += amount,
      Direction::Up => vert_pos -= amount,
    }
  }

  horz_pos * vert_pos
}

pub fn calc_aim(input: Vec<String>) -> i32 {
  let mut horz_pos = 0;
  let mut vert_pos = 0;
  let mut aim = 0;

  for cmd in input {
    let (direction, amount) = format_command(cmd);

    match direction {
      Direction::Forward => {
        horz_pos += amount;
        vert_pos += aim * amount;
      }
      Direction::Down => aim += amount,
      Direction::Up => aim -= amount,
    }
  }

  horz_pos * vert_pos
}

fn format_command(cmd: String) -> (Direction, i32) {
  let mut separated = cmd.split(" ");
  let direction: Direction = Direction::from_str(separated.next().expect("No direction found"))
    .expect("Can't parse direction");
  let amount: i32 = separated
    .next()
    .expect("No amount found")
    .parse()
    .expect("Can't parse amount");
  (direction, amount)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils::get_lines;
  #[test]
  fn test_calc_position() {
    assert_eq!(calc_position(get_lines("src/day2/2-dummy.txt")), 150);
  }

  #[test]
  fn test_calc_aim() {
    assert_eq!(calc_aim(get_lines("src/day2/2-dummy.txt")), 900);
  }
}
