use std::io;
use std::io::Read;
mod mathlib;
use mathlib::*;

fn main() {
  // Buffer to store the whole input
  let mut buffer = String::new();

  // Read the whole input
  io::stdin().read_to_string(&mut buffer).expect("Read error");

  // Split the input by whitespace and parse
  let numbers: Vec<Dec> = buffer
    .split_whitespace()
    .map(|s| s.parse().expect("Parsing error - invalid number"))
    .collect();

  // Calculate the standard deviation
  let mut sum = Dec::zero();
  let mut sd = Dec::zero();
  // for number in numbers.iter() {
  //   sum = add(sum, *number);
  // }
  // sum = divide(sum, numbers.len() as f64);
  // for number in numbers.iter() {
  //   sd = add(sd)
  // }

  for num in numbers {
    println!("{}", num);
  }

  println!("Hello world!");
}
