use day1::{compare_rolling_window, compare_two};
use utils::get_input;

mod day1;
mod utils;

fn main() {
  println!("total: {}", compare_two(get_input("1-1-input.txt")));
  println!(
    "total: {}",
    compare_rolling_window(get_input("1-1-input.txt"))
  );
}
