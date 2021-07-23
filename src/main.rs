extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn app() {
  let mut guess = String::new();
  let secret_number = rand::thread_rng().gen_range(1, 101);

  io::stdin().read_line(&mut guess).expect("Failed to read line");

  let guess: u32 = guess.trim().parse().expect("Expected number");

  println!("You guessed {}", guess);
  println!("Number: {}", secret_number);

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Less"),
    Ordering::Greater => println!("Greater"),
    Ordering::Equal => println!("Equal")
  }
}

fn main() {
  app();
}