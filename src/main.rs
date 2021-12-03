use std::env;

use day1::{compare_rolling_window, compare_two};
use day2::{calc_aim, calc_position};
use utils::{get_input, get_lines};

mod day1;
mod day2;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("pick a day");
    return;
  }

  match args[1].as_str() {
    "day1-1" => println!("1-1: total: {}", compare_two(get_input("1-1-input.txt"))),
    "day1-2" => println!(
      "1-2: total: {}",
      compare_rolling_window(get_input("1-1-input.txt"))
    ),
    "day2-1" => println!("day2: {}", calc_position(get_lines("src/day2/2-input.txt"))),
    "day2-2" => println!("day2: {}", calc_aim(get_lines("src/day2/2-input.txt"))),
    _ => println!("U wot"),
  }
}
