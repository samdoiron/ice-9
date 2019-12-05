extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{self, Read};

use clap::{App, SubCommand};

mod ops;
mod parser;
mod ast;
mod compiler;

use parser::parse;
use compiler::compile;

fn show_ast() {
  let mut code = String::new();
  io::stdin().read_to_string(&mut code).expect("read input");

  let ast = parse(code);
  println!("{:#?}", ast);
}

fn show_components() {
  let mut code = String::new();
  io::stdin().read_to_string(&mut code).expect("read input");

  let ast = parse(code);
  let program = compile(ast);

  println!("Constants: {}", program.constants);
  println!("Text: {}", program.text);
}

fn show_compiled_liquid() {
  let mut code = String::new();
  io::stdin().read_to_string(&mut code).expect("read input");

  let ast = parse(code);
  let program = compile(ast);

  let hydro_template = include_str!("../hydro.liquid");
  let hydro = hydro_template
    .replace("__CONSTANTS__", &program.constants)
    .replace("__BYTECODE__", &program.text);

  println!("{}", hydro);
}

fn main() {
  let app = App::new("plasma")
    .version("0.1")
    .about("Compiles Plasma code")
    .author("Sam Doiron")
    .subcommand(
      SubCommand::with_name("ast")
        .about("Show the parsed AST of the code"))
    .subcommand(
      SubCommand::with_name("components")
        .about("Show the separate components of the compiled output."))
    .subcommand(
      SubCommand::with_name("compile")
        .about("Compile Plasma code to Liquid"));
  let global_matches = app.get_matches();

  if global_matches.subcommand_matches("ast").is_some() {
    show_ast();
    return;
  } else if global_matches.subcommand_matches("components").is_some() {
    show_components();
    return;
  } else if global_matches.subcommand_matches("compile").is_some() {
    show_compiled_liquid();
    return;
  } else {
    println!("{}", "Malformed arguments. See --help for usage.");
    ::std::process::exit(1);
  }
}
