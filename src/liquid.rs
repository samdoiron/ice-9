extern crate liquid;

use std::io::{self, Read};

fn main() {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source);

    let template = liquid::ParserBuilder::with_liquid()
        .build().expect("build parser")
        .parse(&source)
        .expect("parse template");

    let mut globals = liquid::value::Object::new();
    let rendered = template.render(&globals).expect("render");

    println!("{}", rendered);
}
