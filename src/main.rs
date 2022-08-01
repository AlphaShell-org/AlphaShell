#![warn(clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

use std::{
  env,
  fs::{self, File},
  io::{prelude::Read, Write},
  ops::ControlFlow,
  path::Path,
  time::Instant,
};

mod types;

mod tokenize;
use tokenize::tokenize;

mod parse;
use parse::parse;

mod transpile;
use transpile::transpile;

#[macro_use]
mod utils;

fn main() {
  let files = env::args().skip(1);

  let build_path = Path::new("./build");

  if build_path.exists() {
    match fs::remove_dir_all(build_path) {
      Ok(_) => (),
      Err(e) => panic!("Cannot remove folder '{build_path:?}', error: '{e}'"),
    };
  }

  match fs::create_dir_all(build_path) {
    Ok(_) => (),
    Err(e) => panic!("Cannot create folder '{build_path:?}', error: '{e}'"),
  };

  for path in files {
    if let ControlFlow::Break(error) = run_for_file(Path::new(&path), Path::new("./build")) {
      eprintln!("{error}");
      return;
    }
  }
}

fn run_for_file(input_file: &Path, output_path: &Path) -> ControlFlow<String> {
  let start = Instant::now();

  let mut new_path = output_path.join(input_file.file_name().unwrap());
  new_path.set_extension("zsh");

  println!("\nTranspiling {input_file:?} => {new_path:?}");

  let contents = read_file(input_file);

  let tokens = match tokenize(&contents) {
    Ok(tokens) => tokens,
    Err(e) => return ControlFlow::Break(format!("{e}")),
  };

  let tree = match parse(&tokens) {
    Ok(tree) => tree,
    Err(e) => return ControlFlow::Break(format!("{e}")),
  };

  let code = match transpile(&tree) {
    Ok(code) => code,
    Err(e) => return ControlFlow::Break(format!("{e}")),
  };

  write_file(&new_path, &code);

  let duration = start.elapsed();

  println!("\nDone in: {duration:?}\n");

  ControlFlow::Continue(())
}

fn read_file(path: &Path) -> String {
  let mut file = match File::open(path) {
    Ok(path) => path,
    Err(e) => panic!("Couldn't open file '{path:?}', error: '{e}'"),
  };

  let mut contents = String::new();

  match file.read_to_string(&mut contents) {
    Ok(_) => (),
    Err(e) => panic!("Error reading file '{path:?}', error: '{e}'"),
  };

  contents
}

fn write_file(path: &Path, contents: &str) {
  let mut file = match File::create(path) {
    Ok(file) => file,
    Err(e) => panic!("Couldn't open file '{path:?}' for writing, error: '{e}'"),
  };

  match file.write_all(contents.as_bytes()) {
    Ok(_) => (),
    Err(e) => panic!("Error writing file '{path:?}', error: '{e}'"),
  };
}
