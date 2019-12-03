
#[derive(Debug)]
pub enum Expression {
  Number(i64),
  Ident(String),
  Add(Box<Expression>, Box<Expression>),
  Or(Box<Expression>, Box<Expression>),
  And(Box<Expression>, Box<Expression>),
  Not(Box<Expression>),
  Subtract(Box<Expression>, Box<Expression>),
  Divide(Box<Expression>, Box<Expression>),
  Multiply(Box<Expression>, Box<Expression>),
  Modulo(Box<Expression>, Box<Expression>),
  IsGreaterThan(Box<Expression>, Box<Expression>),
  IsLessThan(Box<Expression>, Box<Expression>),
  FunctionCall(String),
  IsEqual(Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub enum Statement {
  Assignment(String, Expression),
  Expression(Expression),
  If(Expression, Vec<Statement>),
  While(Expression, Vec<Statement>),
  Echo(Expression),
  FunctionDefinition(String, Vec<Statement>)
}