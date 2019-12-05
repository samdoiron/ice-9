extern crate liquid;

use std::io::{self, Read};

fn main() {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source).expect("read input");

    let result = liquid::ParserBuilder::with_liquid()
        .build().expect("build parser")
        .parse(&source);

    match result {
      Ok(template) => {
        let globals = liquid::value::Object::new();
        let rendered = template.render(&globals);
        match rendered {
          Ok(output) => {
            println!("{}", output);
          }
          Err(error) => {
            println!("{}", error);
          }
        }
      },
      Err(error) => {
        println!("{}", error);
      }
  }
}
