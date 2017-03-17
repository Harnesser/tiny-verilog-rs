/// Verilog Expression


#[derive(PartialEq, Debug)]
pub enum Operand {
    Literal(i32),
    Identifier(String),
}

pub enum Expression {
    Const(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
}

pub enum Statement {
    Delay             {dly: u32},
    BlockingAssign    {id: Operand, expr: Expression},
    NonBlockingAssign {id: Operand, expr: Expression},
}


// Procedure
pub enum ProcedureType {
    Initial,
    Always,
}

pub struct Procedure {
    pub kind    : ProcedureType,
    pub counter : u32,
    pub stmts   : Vec<Statement>,
}






