#![warn(clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

#[macro_use]
extern crate fstrings;

use std::env;
use std::{fs::File, io::prelude::Read};

mod types;

mod tokenize;
use tokenize::tokenize;

mod parse;
use parse::parse;

#[macro_use]
mod utils;

fn main() {
  let files = env::args().skip(1);

  for path in files {
    let contents = read_file(&path);

    let tokens = match tokenize(&contents) {
      Ok(tokens) => tokens,
      Err(e) => {
        println!("{}", e);
        return;
      }
    };

    println!(
      "{}",
      tokens
        .iter()
        .map(|t| f!("{t}"))
        .collect::<Vec<_>>()
        .join(",\n")
    );

    let tree = match parse(&tokens) {
      Ok(tree) => tree,
      Err(e) => {
        println!("{}", e);
        return;
      }
    };

    println!("{:?}", tree);
  }
}

fn read_file(path: &str) -> String {
  let mut file = match File::open(path) {
    Ok(path) => path,
    Err(e) => panic!("Couldn't open file '{}', error: {}", path, e),
  };
  let mut contents = String::new();
  match file.read_to_string(&mut contents) {
    Ok(_) => (),
    Err(e) => panic!("Error reading file '{}', error: {}", path, e),
  };
  contents
}
