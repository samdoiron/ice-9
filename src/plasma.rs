extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::{self, Read};

use std::borrow::Borrow;
use pest::Parser;
use std::fmt::Write;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PlasmaParser;

type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Debug)]
enum Expression {
  Number(i64),
  Ident(String),
  Add(Box<Expression>, Box<Expression>),
  Or(Box<Expression>, Box<Expression>),
  And(Box<Expression>, Box<Expression>),
  Subtract(Box<Expression>, Box<Expression>),
  IsGreaterThan(Box<Expression>, Box<Expression>),
  IsLessThan(Box<Expression>, Box<Expression>),
  IsEqual(Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
enum Statement {
  Assignment(String, Expression),
  Expression(Expression),
  If(Expression, Vec<Statement>),
  While(Expression, Vec<Statement>),
  Echo(Expression)
}

#[derive(Debug)]
enum Op {
  Constant(usize),
  Echo,
  Add,
  Subtract,
  IsGreaterThan,
  IsLessThan,
  IsEqual,
  And,
  Or,
  Not,
  SetVariable(usize),
  LoadVariable(usize),
  JumpIf(usize),
  Goto(usize),
  Return
}

fn parse<'a>(pairs: Pairs<'a>) -> Vec<Statement> {
  let mut statements = Vec::new();

  for pair in pairs {
    match pair.as_rule() {
      Rule::expression => {
        statements.push(Statement::Expression(parse_expression(pair)));
      },
      Rule::assignment => {
        statements.push(parse_assignment(pair));
      },
      Rule::conditional => {
        statements.push(parse_if(pair));
      }
      Rule::while_loop => {
        statements.push(parse_while(pair));
      },
      Rule::echo => {
        statements.push(parse_echo(pair));
      }
      Rule::EOI => (),
      other => panic!("unknown pair type: {:?}", other)
    }
  }

  statements
}

fn parse_expression(pair: Pair) -> Expression {
  let inner = pair.into_inner().next().unwrap();

  match inner.as_rule() {
    Rule::infix_expression => parse_infix_expression(inner),
    Rule::num_literal => parse_num_literal(inner),
    Rule::non_infix_expression => parse_expression(inner),
    Rule::ident => Expression::Ident(inner.as_str().into()),
    other => panic!("unknown expression rule: {:?}", other)
  }
}

fn parse_assignment(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let var_name = inner.next().unwrap();
  let value = parse_expression(inner.next().unwrap());
  Statement::Assignment(var_name.as_str().into(), value)
}

fn parse_if(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let condition = parse_expression(inner.next().unwrap());
  let body = parse(inner.next().unwrap().into_inner());
  Statement::If(condition, body)
}

fn parse_while(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let condition = parse_expression(inner.next().unwrap());
  let body = parse(inner.next().unwrap().into_inner());
  Statement::While(condition, body)
}

fn parse_echo(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let expression = parse_expression(inner.next().unwrap());
  Statement::Echo(expression)
}


fn parse_num_literal(pair: Pair) -> Expression {
  Expression::Number(pair.as_str().parse().unwrap())
}

fn parse_infix_expression(pair: Pair) -> Expression {
  let mut inner = pair.into_inner();
  let term_one = inner.next().unwrap();
  let op = inner.next().unwrap();
  let term_two = inner.next().unwrap();

  match op.as_str() {
    "+" =>
      Expression::Add(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "-" =>
      Expression::Subtract(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    ">" =>
      Expression::IsGreaterThan(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "<" =>
      Expression::IsLessThan(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "==" =>
      Expression::IsEqual(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "|" =>
      Expression::Or(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "&" =>
      Expression::And(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    other => panic!("unknown infox op: {}", other)
  }
}

struct Program {
  constants: Vec<i64>,
  ops: Vec<Op>,
  variables: Vec<String>
}

impl Program {
  fn compile(statements: Vec<Statement>) -> Program {
    let mut program = Program {
      constants: Vec::new(),
      ops: Vec::new(),
      variables: Vec::new()
    };
    program.compile_program(statements);
    program
  }

  fn compile_program(&mut self, statements: Vec<Statement>) {
    self.compile_statements(&statements);
    self.ops.push(Op::Return);
  }

  fn compile_statements(&mut self, nodes: &Vec<Statement>) {
    for node in nodes {
      self.compile_statement(&node);
    }
  }

  fn compile_statement(&mut self, statement: &Statement)  {
    match statement {
      Statement::Expression(expression) => self.compile_expression(expression),
      Statement::If(condition, body) => self.compile_if(condition, body),
      Statement::While(condition, body) => self.compile_while(condition, body),
      Statement::Echo(expression) => self.compile_echo(expression),
      Statement::Assignment(name, value) => {
        let index = match self.variables.iter().position(|var| var == name) {
          Some(index) => index,
          None => {
            self.variables.push(name.to_owned());
            self.variables.len() - 1
          }
        };
        self.compile_expression(value);
        self.ops.push(Op::SetVariable(index));
      }
    }
  }

  fn compile_if(&mut self, condition: &Expression, body: &Vec<Statement>) {
    // Compiled output looks something like this
    // 
    // <condition>
    // jump_if <if_body_index>
    // <else_body>
    // goto <end_index>
    // <if_body>
    // <end>

    self.compile_expression(condition);
    self.ops.push(Op::JumpIf(0));
    let if_body_jump_index = self.ops.len() - 1;

    // else body fallthrough
    // (empty for now)

    // unconditional jump to END
    self.ops.push(Op::Goto(0));
    let else_jump_index = self.ops.len() - 1;

    // if case
    self.compile_statements(body);

    let end_index = self.ops.len() - 1;
    self.ops[if_body_jump_index] = Op::JumpIf(else_jump_index + 1);
    self.ops[else_jump_index] = Op::Goto(end_index + 1);
  }

  fn compile_while(&mut self, condition: &Expression, body: &Vec<Statement>) {
    // Compiled output looks something like this
    // 
    // <start>
    // <condition>
    // not
    // jump_if <end>
    // <body>
    // goto <start>
    // <if_body>
    // <end>

    let start_index = self.ops.len();
    self.compile_expression(condition);
    self.ops.push(Op::Not);
    self.ops.push(Op::JumpIf(0));
    let end_jump_index = self.ops.len() - 1;

    self.compile_statements(body);

    self.ops.push(Op::Goto(start_index));

    let end_index = self.ops.len();
    self.ops[end_jump_index] = Op::JumpIf(end_index);
  }

  fn compile_echo(&mut self, expression: &Expression) {
    self.compile_expression(expression);
    self.ops.push(Op::Echo);
  }

  fn compile_expression(&mut self, expression: &Expression) {
    match expression {
      Expression::Add(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::Add);
      },
      Expression::Subtract(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::Subtract);
      },
      Expression::IsEqual(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::IsEqual);
      },
      Expression::IsGreaterThan(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::IsGreaterThan);
      },
      Expression::IsLessThan(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::IsLessThan);
      },
      Expression::And(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::And);
      },
      Expression::Or(one, two) => {
        self.compile_expression(one.borrow());
        self.compile_expression(two.borrow());
        self.ops.push(Op::Or);
      },
      Expression::Ident(name) => {
        match self.variables.iter().position(|var| var == name) {
          Some(index) => {
            self.ops.push(Op::LoadVariable(index));
          },
          None => panic!("undefined variable: {}", name)
        }
      },
      Expression::Number(value) => {
        self.constants.push(*value);
        let index = self.constants.len() - 1;
        self.ops.push(Op::Constant(index));
      },
    }
  }
}

fn ops_to_bytecode(ops: Vec<Op>) -> String {
  let mut bytecode = String::new();
  for op in ops {
    match op {
      Op::Echo => write!(&mut bytecode, "e "),
      Op::Constant(index) => write!(&mut bytecode, "c/{} ", index),
      Op::Add => write!(&mut bytecode, "+ "),
      Op::Subtract => write!(&mut bytecode, "- "),
      Op::IsGreaterThan => write!(&mut bytecode, "> "),
      Op::IsLessThan => write!(&mut bytecode, "< "),
      Op::IsEqual => write!(&mut bytecode, "= "),
      Op::And => write!(&mut bytecode, "& "),
      Op::Or => write!(&mut bytecode, "| "),
      Op::Not => write!(&mut bytecode, "! "),
      Op::SetVariable(index) => write!(&mut bytecode, "s/{} ", index),
      Op::LoadVariable(index) => write!(&mut bytecode, "v/{} ", index),
      Op::JumpIf(index) => write!(&mut bytecode, "j/{} ", index),
      Op::Goto(index) => write!(&mut bytecode, "g/{} ", index),
      Op::Return => write!(&mut bytecode, "r ")
    }.expect("compose byetcode");
  }
  bytecode
}

fn main() {
  let mut code = String::new();
  io::stdin().read_to_string(&mut code).expect("read input");

  let pairs = PlasmaParser::parse(Rule::program, &code)
    .unwrap_or_else(|e| panic!("{}", e));

  let hydro_template = include_str!("../hydro.liquid");

  let ast = parse(pairs);
  let program = Program::compile(ast);
  
  let mut constants = String::new();
  for constant in program.constants {
    write!(&mut constants, "{} ", constant).expect("write constants");
  }

  let hydro = hydro_template
    .replace("__CONSTANTS__", &constants)
    .replace("__BYTECODE__", &ops_to_bytecode(program.ops));

  println!("{}", hydro)
}