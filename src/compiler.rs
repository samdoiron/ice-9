use std::collections::HashMap;

use ops::Op;
use ast::{Statement, Expression};

use std::fmt::Write;

type SymbolTable = Vec<(String, Vec<Op>)>;

struct StatementChunk {
  constants: Vec<i64>,
  ops: Vec<Op>,
  variables: Vec<String>,
  symbol_table: SymbolTable
}

struct CompiledChunk {
  constants: Vec<i64>,
  ops: Vec<Op>,
  symbol_table: SymbolTable
}

impl StatementChunk {
  fn new() -> StatementChunk {
    StatementChunk {
      ops: Vec::new(),
      constants: Vec::new(),
      variables: Vec::new(),
      symbol_table: Vec::new()
    }
  }

  fn compile(mut self, statements: Vec<Statement>) -> CompiledChunk {
      self.compile_statements(statements);
      self.ops.push(Op::Return);
      CompiledChunk {
        constants: self.constants,
        ops: self.ops,
        symbol_table: self.symbol_table
      }
  }

  fn compile_statements(&mut self, statements: Vec<Statement>) {
    for statement in statements {
      self.compile_statement(statement);
    }
  }

  fn compile_statement(&mut self, statement: Statement)  {
    match statement {
      Statement::Expression(expression) => self.compile_expression(expression),
      Statement::If(condition, body) => self.compile_if(condition, body),
      Statement::While(condition, body) => self.compile_while(condition, body),
      Statement::Echo(expression) => self.compile_echo(expression),
      Statement::Assignment(name, value) => {
        let index = match self.variables.iter().position(|var| var == &name) {
          Some(index) => index,
          None => {
            self.variables.push(name.into());
            self.variables.len() - 1
          }
        };
        self.compile_expression(value);
        self.ops.push(Op::SetVariable(index));
      },
      Statement::FunctionDefinition(name, body) => {
        self.compile_function_definition(name, body)
      }
    }
  }

  fn compile_if(&mut self, condition: Expression, body: Vec<Statement>) {
    // Compiled output looks something like this
    // 
    // <condition>
    // <if_body_jump_index> │ jump_if <if_body_index> >─╮
    // ...                  │ <else_body>               │
    // <end_jump_index>     │ goto <end_index> >──────────╮
    // <if_body_index>      │ <if_body>            <────╯ │
    // ...                  │ <if_body>                   │
    // <end_index>          │ ... whatever is next <──────╯

    self.compile_expression(condition);

    let if_body_jump_index = self.ops.len() as isize;
    self.ops.push(Op::JumpIf(0));

    // else body fallthrough
    // (empty for now)

    let end_jump_index = self.ops.len() as isize;
    self.ops.push(Op::Goto(0));

    let if_body_index = self.ops.len() as isize;
    self.compile_statements(body);

    let end_index = self.ops.len() as isize;
    self.ops[if_body_jump_index as usize] = Op::JumpIf(if_body_index - if_body_jump_index);

    self.ops[end_jump_index as usize] = Op::Goto(end_index - end_jump_index);
  }

  fn compile_while(&mut self, condition: Expression, body: Vec<Statement>) {
    // Compiled output looks something like this
    // 
    // <start_index>    | <condition> <───────────────╮
    // ...              | <condition>                 │
    // ...              | not                         │
    // ...              | jump_if <end_index>  >────╮ │
    // ...              | <body>                    │ │
    // ...              | goto <start_index>   >────│─╯
    // <end_index>      | ... whatever comes next <─╯

    let start_index = self.ops.len();
    self.compile_expression(condition);
    self.ops.push(Op::Not);
    self.ops.push(Op::JumpIf(0));
    let end_jump_index = self.ops.len() - 1;

    self.compile_statements(body);

    self.ops.push(Op::Goto(start_index as isize - self.ops.len() as isize));

    let end_index = self.ops.len();
    self.ops[end_jump_index] = Op::JumpIf(end_index as isize - end_jump_index as isize);
  }

  fn compile_echo(&mut self, expression: Expression) {
    self.compile_expression(expression);
    self.ops.push(Op::Echo);
  }

  fn compile_function_definition(&mut self, name: String, body: Vec<Statement>) {
    let mut compiled_chunk = StatementChunk::new().compile(body);
    self.constants.append(&mut compiled_chunk.constants);
    self.symbol_table.push((name.into(), compiled_chunk.ops));
  }

  fn compile_expression(&mut self, expression: Expression) {
    match expression {
      Expression::Add(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::Add);
      },
      Expression::Subtract(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::Subtract);
      },
      Expression::Multiply(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::Multiply);
      },
      Expression::Divide(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::Divide);
      },
      Expression::Modulo(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::Modulo);
      },
      Expression::IsEqual(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::IsEqual);
      },
      Expression::IsGreaterThan(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::IsGreaterThan);
      },
      Expression::IsLessThan(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::IsLessThan);
      },
      Expression::And(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::And);
      },
      Expression::Or(one, two) => {
        self.compile_expression(*one);
        self.compile_expression(*two);
        self.ops.push(Op::Or);
      },
      Expression::Not(inner) => {
        self.compile_expression(*inner);
        self.ops.push(Op::Not);
      },
      Expression::Ident(name) => {
        match self.variables.iter().position(|var| var == &name) {
          Some(index) => {
            self.ops.push(Op::LoadVariable(index));
          },
          None => panic!("undefined variable: {}", name)
        }
      },
      Expression::Number(value) => {
        self.constants.push(value);
        let index = self.constants.len() - 1;
        self.ops.push(Op::Constant(index));
      },
      Expression::FunctionCall(name) => {
        self.ops.push(Op::CallFunction(name.into()));
      }
    }
  }
}

pub struct Program {
  pub constants: String,

  // Includes main followed by function
  pub text: String
}

pub fn compile(statements: Vec<Statement>) -> Program { 
  let compiled_chunk = StatementChunk::new().compile(statements);

  let CompiledChunk {
    ops: mut program_ops,
    constants,
    symbol_table,
    ..
  } = compiled_chunk;

  let mut offsets = HashMap::new();

  // First pass: construct offset table, concat functions on to text
  for (name, mut ops) in symbol_table {
    offsets.insert(name, program_ops.len());
    program_ops.append(&mut ops);
  }

  // Second pass: construct actual bytecode, replacing names with offsets
  let mut text = String::new();
  for op in program_ops {
    match op {
      Op::Echo => write!(&mut text, "e "),
      Op::Constant(index) => write!(&mut text, "c/{} ", index),
      Op::Add => write!(&mut text, "+ "),
      Op::Subtract => write!(&mut text, "- "),
      Op::Multiply => write!(&mut text, "* "),
      Op::Divide => write!(&mut text, "÷ "),
      Op::Modulo => write!(&mut text, "% "),
      Op::IsGreaterThan => write!(&mut text, "> "),
      Op::IsLessThan => write!(&mut text, "< "),
      Op::IsEqual => write!(&mut text, "= "),
      Op::And => write!(&mut text, "& "),
      Op::Or => write!(&mut text, "| "),
      Op::Not => write!(&mut text, "! "),
      Op::SetVariable(index) => write!(&mut text, "s/{} ", index),
      Op::LoadVariable(index) => write!(&mut text, "v/{} ", index),
      Op::JumpIf(index) => write!(&mut text, "j/{} ", index),
      Op::Goto(index) => write!(&mut text, "g/{} ", index),
      Op::Return => write!(&mut text, "r "),
      Op::CallFunction(name) => {
        match offsets.get(&name) {
          Some(offset) => {
            write!(&mut text, "k/{} ", offset)
          },
          None => panic!("No definition found for function {}", name)
        }
      }
    }.expect("compose text");
  }

  let mut constant_str = String::new();
  for constant in constants {
    write!(&mut constant_str, "{} ", constant).expect("write constants");
  }

  Program {
    text: text,
    constants: constant_str
  }
}
