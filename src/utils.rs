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

pub fn get_input(filename: &str) -> Vec<i32> {
  let lines = read_lines(filename).expect("Somehow failed to open the file");
  lines.map(|line| line.unwrap().parse().unwrap()).collect()
}
