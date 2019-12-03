
#[derive(Debug)]
pub enum Op {
  Constant(usize),
  Echo,
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  IsGreaterThan,
  IsLessThan,
  IsEqual,
  And,
  Or,
  Not,
  SetVariable(usize),
  LoadVariable(usize),
  JumpIf(isize),
  Goto(isize),

  // Filled in with the correct location at compile time based on the value
  // found in the name table
  CallFunction(String),
  Return
}
