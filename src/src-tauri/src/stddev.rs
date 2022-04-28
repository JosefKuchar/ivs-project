//! A module to calculate standard deviation
//!
//! Input are numbers split by whitespace specified in stdin.
//! Output is standard deviation printed to stdout.

use std::io;
use std::io::Read;
mod mathlib;
use mathlib::*;

/// Entry point
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

  // Variables
  let n = Dec::from(numbers.len());
  let mut avg = Dec::zero();
  let mut sum = Dec::zero();
  let mut tmp;

  // Standard deviation of 1 number is 0
  if n == Dec::from(1) {
    println!("0");
    return;
  }

  // Calculate the average
  for numbers in numbers.iter() {
    avg = add(avg, *numbers);
  }

  avg = divide(avg, n).expect("Cannot calculate average of 0 elements!");

  // Calculate specified sum
  for number in numbers.iter() {
    tmp = subtract(*number, avg);
    tmp = pow(tmp, Dec::from(2)).expect("Error calculating standard deviation!");
    sum = add(sum, tmp);
  }

  // Calculate the result
  tmp = subtract(n, Dec::from(1));
  tmp = divide(Dec::from(1), tmp).expect("Error calulating standard deviation!");
  tmp = multiply(tmp, sum);
  let result = root(tmp, Dec::from(2)).expect("Error calculating standard deviation!");

  println!("{}", result);
}
