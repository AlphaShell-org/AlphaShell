#![warn(clippy::pedantic)]

use std::{
  fs::{self, File},
  io::{self, prelude::Read, Write},
  path::{Path, PathBuf},
  process,
  time::Instant,
};

use clap::{ArgAction, Parser};

mod types;

mod tokenize;
use tokenize::tokenize;

mod parse;
use parse::parse;

mod transpile;
use transpile::transpile;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
  /// List of input files to be transpiled
  ///
  /// Use '-' to read from stdin.
  /// Use '--' to signify end of flags and allow file names like '-v' to be used.
  files: Vec<PathBuf>,

  /// Name of the output file
  ///
  /// Ignored when multiple files are given.
  /// Overrides --output-directory for single file.
  /// Use '-' to output result to stdout (requires --quiet).
  /// If no output file is given, input file name with '.zsh' extension is used instead
  #[clap(short, long)]
  output: Option<PathBuf>,

  /// Name of the output directory
  ///
  /// Gets overridden by --output for single file.
  /// If no directory is given, './' is used instead"
  #[clap(short = 'd', long)]
  output_directory: Option<PathBuf>,

  /// Suppress info output
  #[clap(short, long, action = ArgAction::Count)]
  quiet: u8,

  /// Verbose output
  #[clap(short, long, action = ArgAction::Count)]
  verbose: u8,
}

macro_rules! error {
  ($($arg:tt)*) => {{
      eprintln!($($arg)*);
      process::exit(1)
  }};
}

fn verbosity(args: &Args) -> i32 {
  i32::from(args.verbose) - i32::from(args.quiet)
}

macro_rules! warn {
  ($args:tt, $($arg:tt)*) => {{
    if verbosity($args) >= -1 {
      println!($($arg)*);
    }
  }};
}

macro_rules! info {
  ($args:tt, $($arg:tt)*) => {{
    if verbosity($args) >= 0 {
      println!($($arg)*);
    }
  }};
}

macro_rules! verbose {
  ($args:tt, $($arg:tt)*) => {{
    if verbosity($args) >= 1 {
      println!($($arg)*);
    }
  }};
}

fn main() {
  let args = Args::parse();

  if args.files.is_empty() {
    error!("No files specified");
  }

  if args.files.len() == 1 {
    run_single(&args);
  } else {
    run_multiple(&args);
  }
}

fn run_single(args: &Args) {
  let file = &args.files[0];

  let output_path = if let Some(output) = &args.output {
    output.clone()
  } else if let Some(output_directory) = &args.output_directory {
    create_directory(args, output_directory);

    output_directory
      .join(file.file_name().unwrap())
      .with_extension("zsh")
  } else {
    file.with_extension("zsh")
  };

  let input: Box<dyn io::Read> = if is_std_io(file) {
    Box::new(io::stdin())
  } else {
    Box::new(open_file(args, file))
  };

  let output: Box<dyn io::Write> = if is_std_io(&output_path) {
    warn_if_not_quiet(args);
    Box::new(io::stdout())
  } else {
    Box::new(create_file(args, &output_path))
  };

  run_for_file(args, file, input, output);
}

fn run_multiple(args: &Args) {
  let output_directory = args
    .output_directory
    .clone()
    .unwrap_or_else(|| PathBuf::from("."));

  create_directory(args, &output_directory);

  for file in &args.files {
    let input: Box<dyn io::Read> = if is_std_io(file) {
      Box::new(io::stdin())
    } else {
      Box::new(open_file(args, file))
    };

    let output_path = output_directory
      .join(file.file_name().unwrap())
      .with_extension("zsh");

    let output: Box<dyn io::Write> = Box::new(create_file(args, &output_path));

    run_for_file(args, file, input, output);
  }
}

fn run_for_file(
  args: &Args,
  file_name: &Path,
  mut input: Box<dyn Read>,
  mut output: Box<dyn Write>,
) {
  // use closure to better handle errors
  let mut inner = move || -> io::Result<()> {
    let input: &mut dyn io::Read = &mut input;
    let output: &mut dyn io::Write = &mut output;
    let contents = read(input)?;

    let tokens = tokenize(&contents).unwrap_or_else(|e| error!("{e}"));
    let tree = parse(&tokens).unwrap_or_else(|e| error!("{e}"));
    let code = transpile(&tree).unwrap_or_else(|e| error!("{e}"));

    writeln!(output, "{code}")?;

    Ok(())
  };

  let start = Instant::now();
  let result = inner();
  let elapsed = start.elapsed();

  if let Err(error) = result {
    error!("{error}");
  }

  info!(args, "Transpiled '{}' in {elapsed:?}", file_name.display());
}

fn is_std_io(path: &Path) -> bool {
  path.to_string_lossy() == "-"
}

fn warn_if_not_quiet(args: &Args) {
  if verbosity(args) >= 0 {
    warn!(args, "Outputting to stdout without --quiet!");
  }
}

fn create_directory(args: &Args, directory: &Path) {
  verbose!(args, "Creating directory '{}'", directory.display());
  if let Err(e) = fs::create_dir_all(directory) {
    error!(
      "Couldn't create directory '{}', error: '{e}'",
      directory.display()
    );
  };
}

fn open_file(args: &Args, path: &Path) -> File {
  verbose!(args, "Opening file: {}", path.display());
  File::open(path)
    .unwrap_or_else(|e| error!("Couldn't open file '{}', error: '{e}'", path.display()))
}

fn create_file(args: &Args, path: &Path) -> File {
  if path.exists() {
    verbose!(args, "Removing file: {}", path.display());
    if let Err(e) = fs::remove_file(path) {
      error!("Couldn't remove file '{}', error: '{e}'", path.display());
    }
  }

  verbose!(args, "Creating file: {}", path.display());
  File::create(path)
    .unwrap_or_else(|e| error!("Couldn't create file '{}', error: '{e}'", path.display()))
}

fn read(source: &mut dyn io::Read) -> io::Result<String> {
  let mut contents = String::new();
  source.read_to_string(&mut contents)?;
  Ok(contents)
}
