/* lets do stuff */
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]

struct Survivor {
  class: &'static str,
  gender: &'static str,
  age: &'static str,
  survived: bool,
  frequency: u16,
}

fn get_file(file_path: &'static str) -> File {
  let path = Path::new(file_path);
  let display = path.display();

  match File::open(&path) {
    Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
    Ok(file) => file,
  }
}

fn stringify(mut file: File) -> String {
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {} ", Error::description(&why)),
    Ok(_) => s
  }
}


fn main() {
  let f = get_file("lib/Titanic.csv");
  let s = stringify(f);
  println!("Titanic {}", s);
}