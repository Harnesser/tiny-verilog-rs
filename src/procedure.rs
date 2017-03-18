/// Verilog Expression

use std::fmt;

pub type Value = usize;
pub type Time = usize;

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum Operand {
    Literal(Value),
    Identifier(String),
}

impl fmt::Display for Operand {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Literal(ref num) => {
                write!(f, "{}", num)
            },
            Operand::Identifier(ref var) => {
                write!(f, "{}", var)
            },
        }
    }
}


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expression {
    Const(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
}

impl fmt::Display for Expression {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Const(ref num) => {
                write!(f, "{}", num)
            },
            Expression::And(ref a, ref b) => {
                write!(f, "{} & {}", a, b)
            },
            Expression::Or(ref a, ref b) => {
                write!(f, "{} | {}", a, b)
            },
            Expression::Not(ref a) => {
                write!(f, "~{}", a)
            },
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement {
    Delay             {dly: Time},
    BlockingAssign    {id: Operand, expr: Expression},
    NonBlockingAssign {id: Operand, expr: Expression},
    AtChange          {ids: Vec<Operand>},
}

impl fmt::Display for Statement {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Delay{ref dly} => {
                write!(f, "#{}", dly)
            },
            Statement::BlockingAssign{ref id, ref expr} => {
                write!(f, "{} = {}", id, expr)
            },
            Statement::NonBlockingAssign{ref id, ref expr} => {
                write!(f, "{} <= {}", id, expr)
            },
            Statement::AtChange{ref ids} => {
                let mut ids_str: Vec<String> = vec![];

                for id in 0..ids.len() {
                    match &ids[id] {
                        &Operand::Identifier(ref id) => {
                            ids_str.push(id.clone());
                        },
                        _ => {}
                    }
                }
                let sensitivity_list = ids_str.join(" or ");
                write!(f, "@({})", sensitivity_list)
            },
        }
    }
}

// Procedure
#[allow(dead_code)]
pub enum ProcedureType {
    Initial,
    Always,
}

impl fmt::Display for ProcedureType {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProcedureType::Initial => write!(f, "INITIAL"),
            ProcedureType::Always => write!(f, "ALWAYS"),
        }
    }
}


pub struct Procedure {
    pub kind    : ProcedureType,
    pub counter : usize,
    pub stmts   : Vec<Statement>,
}

impl Procedure {

    pub fn next_stmt(&mut self) -> Option<Statement> {
        let mut stmt : Option<Statement> = None;
        if self.counter < self.stmts.len() {
            stmt = Some(self.stmts[self.counter].clone());
            self.counter += 1;
        }
        stmt
    }

    pub fn push(&mut self, stmt: Statement ) {
        self.stmts.push(stmt);
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        println!("{}", self.kind);
        for i in 0..self.stmts.len() {
            println!(" {}", self.stmts[i]);
        }
    }
}

