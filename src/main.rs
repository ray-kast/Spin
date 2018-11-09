#![feature(try_from)]

#[macro_use]
extern crate lalrpop_util; // TODO: I think I should be able to remove this

#[macro_use]
extern crate lazy_static;

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

  let root_scope = uwml::builtin::gen_scope();

  println!("root scope: {:#?}", root_scope);

  let doc = uwml::compile(ast, &root_scope);

  println!("document: {:#?}", doc);

  let html = doc.gen_html();

  println!("HTML document: {:#?}", html);

  let html_str = html.to_string();

  println!("HTML string: {}", html_str);
}
