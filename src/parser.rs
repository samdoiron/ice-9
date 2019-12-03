
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PlasmaParser;

type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;

use pest::Parser;

use ast::{Statement, Expression};

pub fn parse(code: String) -> Vec<Statement> {
  let pairs = PlasmaParser::parse(Rule::program, &code)
    .unwrap_or_else(|e| panic!("{}", e));
  parse_statements(pairs)
}

fn parse_statements<'a>(pairs: Pairs<'a>) -> Vec<Statement> {
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
      Rule::func_definition => {
        statements.push(parse_function_definition(pair))
      },
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
    Rule::not => parse_not(inner),
    Rule::expression => parse_expression(inner),
    Rule::func_call => parse_func_call(inner),
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
  let body = parse_statements(inner.next().unwrap().into_inner());
  Statement::If(condition, body)
}

fn parse_while(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let condition = parse_expression(inner.next().unwrap());
  let body = parse_statements(inner.next().unwrap().into_inner());
  Statement::While(condition, body)
}

fn parse_echo(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let expression = parse_expression(inner.next().unwrap());
  Statement::Echo(expression)
}

fn parse_function_definition(pair: Pair) -> Statement {
  let mut inner = pair.into_inner();
  let name = inner.next().unwrap();
  let body = parse_statements(inner.next().unwrap().into_inner());
  Statement::FunctionDefinition(name.as_str().into(), body)
}

fn parse_num_literal(pair: Pair) -> Expression {
  Expression::Number(pair.as_str().parse().unwrap())
}

fn parse_not(pair: Pair) -> Expression {
  let mut inner = pair.into_inner();
  let expression = parse_expression(inner.next().unwrap());
  Expression::Not(Box::new(expression))
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
    "*" =>
      Expression::Multiply(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "/" =>
      Expression::Divide(
        Box::new(parse_expression(term_one)),
        Box::new(parse_expression(term_two)),
      ),
    "%" =>
      Expression::Modulo(
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

fn parse_func_call(pair: Pair) -> Expression {
  Expression::FunctionCall(pair.into_inner().next().unwrap().as_str().into())
}
