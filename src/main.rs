use std::fmt;
use std::iter::Enumerate;

#[derive(Debug)]
enum Op {
    Return,
    Constant(usize),
    Add,
    Echo
}

#[derive(Debug)]
enum Value {
    Double(f64),
}

struct Program {
    constants: Vec<Value>,
    ops: Vec<Op>
}

impl Program {
    fn new() -> Program {
        Program {
            constants: Vec::new(),
            ops: Vec::new()
        }
    }

    fn push_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }

    fn push_op(&mut self, op: Op) {
        self.ops.push(op);
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=== Constants:\n");
        for (index, constant) in self.constants.iter().enumerate() {
            write!(f, "{} = {:?}\n", index, constant)?;
        }
        write!(f, "\n=== Operations:\n");
        for (index, operation) in self.ops.iter().enumerate() {
            write!(f, "{}: {:?}\n", index, operation)?;
        }
        Ok(())
    }
}

fn main() {
    let mut program = Program::new();

    let one = program.push_constant(Value::Double(1f64));
    let two = program.push_constant(Value::Double(2f64));

    program.push_op(Op::Constant(one));
    program.push_op(Op::Constant(two));

    program.push_op(Op::Add);
    program.push_op(Op::Echo);
    program.push_op(Op::Return);

    println!("{:?}", program);
}
