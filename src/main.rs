#![warn(clippy::pedantic)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![feature(let_chains)]

use std::{env, fs::File, io::prelude::Read, ops::ControlFlow, path::Path, time::Instant};

mod types;

mod tokenize;
use tokenize::tokenize;

mod parse;
use parse::parse;

// mod transpile;
// use transpile::transpile;

#[macro_use]
mod utils;

fn main() {
  let files = env::args().skip(1);

  for path in files {
    if let ControlFlow::Break(_) = run_for_file(Path::new(&path)) {
      return;
    }
  }
}

macro_rules! time {
  ($name:expr, $b:block) => {{
    let start = std::time::Instant::now();
    let result = $b;
    let duration = start.elapsed();
    println!("{} done in {duration:?}", $name);
    result
  }};
}

fn run_for_file(path: &Path) -> ControlFlow<()> {
  let start = Instant::now();

  println!("\nTranspiling {path:?}\n");

  let contents = read_file(path);

  let tokens = time!("Lexing", {
    match tokenize(&contents) {
      Ok(tokens) => tokens,
      Err(e) => {
        eprintln!("{e}");
        return ControlFlow::Break(());
      }
    }
  });

  let tree = time!("Parsing", {
    match parse(&tokens) {
      Ok(tree) => tree,
      Err(e) => {
        eprintln!("{e}");
        return ControlFlow::Break(());
      }
    }
  });

  println!("{tree:?}");

  /* let code = time!("Transpiling", {
    match transpile(&tree) {
      Ok(code) => code,
      Err(e) => {
        eprintln!("{e}");
        return ControlFlow::Break(());
      }
    }
  }); */

  /* time!("Writing", {
    let build_path = Path::new("./build");

    if build_path.exists() {
      match fs::remove_dir_all(build_path) {
        Ok(_) => (),
        Err(e) => panic!("Cannot remove folder '{build_path:?}', error: '{e}'"),
      };
    }

    match fs::create_dir_all(build_path) {
      Ok(_) => (),
      Err(e) => panic!("Cannot create folder '{path:?}', error: '{e}'"),
    };

    let mut new_path = build_path.join(path.file_name().unwrap());
    new_path.set_extension("zsh");
    write_file(&new_path, &code);
  }); */

  let duration = start.elapsed();

  println!("\nTotal time for {path:?}: {duration:?}\n");

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

// fn write_file(path: &Path, contents: &str) {
//   let mut file = match File::create(path) {
//     Ok(file) => file,
//     Err(e) => panic!("Couldn't open file '{path:?}' for writing, error: '{e}'"),
//   };

//   match file.write_all(contents.as_bytes()) {
//     Ok(_) => (),
//     Err(e) => panic!("Error writing file '{path:?}', error: '{e}'"),
//   };
// }
