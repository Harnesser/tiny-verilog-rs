/// Verilog Expression

use std::fmt;

pub type Value = usize;
pub type Time = usize;
pub type ProcId = usize;

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

impl Operand {
    pub fn get_identifier(&self) -> Option<String> {
        if let Operand::Identifier(ref var) = *self {
            Some(var.clone())
        } else {
            None
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

impl Expression {
    fn get_identifiers(&self) -> Vec<String> {
        let mut vars: Vec<String> = vec![];
        match *self {
            Expression::Const(ref num) => {
                if let Some(var) = num.get_identifier() {
                    vars.push(var);
                }
            },
            Expression::And(ref a, ref b) | 
            Expression::Or(ref a, ref b) => {
                if let Some(var) = a.get_identifier() {
                    vars.push(var);
                }
                if let Some(var) = b.get_identifier() {
                    vars.push(var);
                }
            },
            Expression::Not(ref a) => {
                if let Some(var) = a.get_identifier() {
                    vars.push(var);
                }
            },
        }
        vars
    }
}



#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Edge {
    Rise(String),  // zero to non-zero
    Fall(String),  // non-zero to zero
    Any(String),   // anything else, eg 1 to 2
}

impl fmt::Display for Edge {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match *self {
            Edge::Any(ref var) => {
                write!(f, "{}", var)
            },
            Edge::Rise(ref var) => {
                write!(f, "posedge {}", var)
            },
            Edge::Fall(ref var) => {
                write!(f, "negedge {}", var)
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
    AtChange          {edges: Vec<Edge>},
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
            Statement::AtChange{ref edges} => {
                let mut ids_str: Vec<String> = vec![];

                for edge in edges {
                    let estr = format!("{}", edge);
                    ids_str.push(estr);
                }
                let sensitivity_list = ids_str.join(" or ");
                write!(f, "@({})", sensitivity_list)
            },
        }
    }
}

impl Statement {
    pub fn get_identifiers(&self) -> Vec<String> {
        let mut vars: Vec<String> = vec![];
        match *self {
            Statement::BlockingAssign{ref id, ref expr} |
            Statement::NonBlockingAssign{ref id, ref expr} => {
                if let Some(var) = id.get_identifier() {
                    vars.push(var.clone());
                }
                vars.append( &mut expr.get_identifiers() );
            },
            _ => {}, // don't care about anything in other statement types
        }
        vars
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
        
        // always can go again...
        if let ProcedureType::Always = self.kind {
            if self.counter == self.stmts.len() {
                self.counter = 0;
            }
        }

        if self.counter < self.stmts.len() {
            stmt = Some(self.stmts[self.counter].clone());
            self.counter += 1;
        }
        stmt
    }

    pub fn push(&mut self, stmt: Statement ) {
        self.stmts.push(stmt);
    }

    pub fn get_identifiers(&self) -> Vec<String> {
        let mut vars: Vec<String> = vec![];
        for stmt in &self.stmts {
            let stmt_vars = stmt.get_identifiers();
            for var in stmt_vars {
                if !vars.contains(&var) {
                    vars.push(var);
                }
            }
        }
        vars
    }

    #[allow(dead_code)]
    pub fn show(&self) {
        println!("{}", self.kind);
        for i in 0..self.stmts.len() {
            println!(" {}", self.stmts[i]);
        }
    }
}

