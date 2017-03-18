/// Verilog Expression


#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum Operand {
    Literal(i32),
    Identifier(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Expression {
    Const(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Statement {
    Delay             {dly: u32},
    BlockingAssign    {id: Operand, expr: Expression},
    NonBlockingAssign {id: Operand, expr: Expression},
}


// Procedure
#[allow(dead_code)]
pub enum ProcedureType {
    Initial,
    Always,
}

pub struct Procedure {
    pub kind    : ProcedureType,
    pub counter : usize,
    pub stmts   : Vec<Statement>,
}

impl Procedure {

    pub fn next(&mut self) -> Option<Statement> {
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
        println!("Procedure");
        for i in 0..self.stmts.len() {
            println!(" {:?}", self.stmts[i]);
        }
    }
}





