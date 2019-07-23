extern crate jsonprima;

use clap::{App, Arg};

fn main() {
  let matches = App::new("JSONPrima")
    .author("George Gkasdrogkas <georgegkas@gmail.com>")
    .about("RFC 8259 compliant JSON validator in Rust.")
    .arg(
      Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("INPUT")
        .help("Load JSON document from CLI")
        .takes_value(true),
    )
    .get_matches();

  if !matches.is_present("input") {
    println!("You should pass one valid CLI argument.");
    std::process::exit(-1);
  }

  let errors = jsonprima::validate(matches.value_of("input").unwrap_or(""));

  if errors.is_empty() {
    std::process::exit(0);
  } else {
    println!("{:#?}", errors);
    std::process::exit(1);
  }
}
