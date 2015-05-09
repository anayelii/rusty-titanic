/* lets do stuff */

#[allow(dead_code)]

struct Survivor {
  class: &'static str,
  gender: &'static str,
  age: &'static str,
  survived: bool,
  frequency: u16,
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  let path = Path::new("lib/Titanic.csv");
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {} ", display, Error::description(&why)),
    Ok(_) => print!(" {} contains:\n{}", display, s),
  }


}