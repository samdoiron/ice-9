
#[derive(Debug)]
enum Op {
  Echo(String)
}

struct Program(Vec<Op>);

struct Bytecode(String);

fn compile(program: Program) -> Bytecode {
  let mut output = Bytecode(String::new());

  for op in program.0 {
    match op {
      Op::Echo(message) => {
        output.0.push_str(&message);
      }
    }
  }

  output
}

fn main() {
  let program = Program(vec![
    Op::Echo("Hello world".into())
  ]);

  println!("{}", compile(program).0);
}
