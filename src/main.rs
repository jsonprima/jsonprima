extern crate jsonprima;

use std::env;
use std::fs;
use std::path::Path;

// TODO: Implement a proper CLI tool using a package like `clap`.

fn main() {
  let args: Vec<_> = env::args().skip(1).collect();

  match args.first() {
    Some(option) => match &option[..] {
      "-f" => {
        let p = Path::new(args.get(1).unwrap());

        if !p.is_file() {
          print!("Not a valid text file.");
        }

        let text = fs::read_to_string(p).expect("Something went wrong reading the file");

        let errors = jsonprima::validate(&text);

        if errors.is_empty() {
          std::process::exit(0);
        } else {
          std::process::exit(1);
        }
      }

      "-t" => {
        let text = args.get(1).unwrap();

        let errors = jsonprima::validate(&text);

        if errors.is_empty() {
          std::process::exit(0);
        } else {
          std::process::exit(1);
        }
      }

      _ => println!("Unrecognized parameter."),
    },

    None => (println!("Specify a mode: '-f' for file, '-t' for text")),
  }
}
