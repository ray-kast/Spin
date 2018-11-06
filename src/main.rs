#[macro_use]
extern crate lalrpop_util; // TODO: I think I should be able to remove this

pub mod ast;
mod uwml;

lalrpop_mod!(pub grammar);

use self::grammar::DocParser;
use clap::{App, Arg};
use std::{fs::File, io::prelude::*, process::exit};

fn main() {
  let matches = App::new("spin")
    .version(env!("CARGO_PKG_VERSION"))
    .about("TODO: put a description here")
    .arg(Arg::with_name("input").required(true))
    .get_matches();

  let s = {
    let mut file = File::open(matches.value_of("input").unwrap()).unwrap();

    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();

    s
  };

  let parser = DocParser::new();

  let ast = match parser.parse(&s) {
    Ok(d) => d,
    Err(e) => {
      println!("an error occured during parsing: {}", e);

      exit(1);
    },
  };

  println!("AST: {:#?}", ast);

  let doc = uwml::compile(ast);

  println!("document: {:#?}", doc);
}
