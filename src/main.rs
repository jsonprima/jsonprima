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
    std::process::exit(1);
  }

  let errors = jsonprima::validate(matches.value_of("input").unwrap_or(""));

  if errors.is_empty() {
    println!("[]");
    std::process::exit(0);
  } else {
    let mut serialized_errors = "[".to_string();
    for error in errors.iter() {
      serialized_errors.push_str(&error.serialize());
      serialized_errors.push_str(",");
    }
    serialized_errors.truncate(serialized_errors.len() - 1);
    serialized_errors.push_str("]");

    println!("{:#?}", serialized_errors);
    std::process::exit(0);
  }
}
