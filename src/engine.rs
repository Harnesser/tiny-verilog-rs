
use procedure::*;

use std::collections::VecDeque;
use std::collections::HashMap;

//use procedure::Value;

pub struct Engine {
    procedures : Vec<Procedure>,
    q_active : VecDeque<Statement>,
    symtable : HashMap<String, Value>,
}


impl Engine {

    pub fn new() -> Engine {
        Engine {
            symtable: HashMap::new(),
            procedures: vec![],
            q_active: VecDeque::new(),
        }
    }

    pub fn run(&mut self) {
        // simulation loop
        println!("\n*INFO* Starting simulation");
        let mut c_loop = 0;
        loop {
            println!("\n*INFO* Loop {}", c_loop);
            self.show_symtable();
            self.show_queues();

            if self.q_active.len() > 0 {
                println!("*INFO* Emptying active queue");
                while let Some(stmt) = self.q_active.pop_back() {
                    self.execute(stmt);
                }
            } else {
                println!("*INFO* Get events from procedures");
                let mut c_stmt = 0;
                for p in &mut self.procedures {
                    match p.next() {
                        Some(stmt) => {
                            println!("*INFO* Loading: {}", stmt);
                            self.q_active.push_front(stmt);
                            c_stmt += 1;
                        }
                        _ => {}
                    }
                }
                if c_stmt == 0 {
                    println!("*INFO* Event starved!");
                    break;
                }
            }
            c_loop += 1;
        }
        println!("\n*INFO* Done");
        self.show_symtable();
    }


    fn execute(&mut self, stmt: Statement) {
        println!("*INFO* Executing: {}", stmt);
        match stmt {

            Statement::BlockingAssign{id, expr} => {
                println!("=");
                match id {
                    Operand::Identifier(i) => {
                        let val = self.evaluate(expr);
                        self.symtable.insert(i, val);
                    },
                    Operand::Literal(i) => {
                    },
                }
            },

            Statement::NonBlockingAssign{id, expr} => {
                println!("*WARNING* \"<=\" not implemented");
            },

            Statement::Delay{dly} => {
                println!("*WARNING* \"Delay\" not implemented");
            },

        }
    }

    fn lookup(&mut self, op: Operand) -> Value {
        match op {
            Operand::Literal(num) => num,
            Operand::Identifier(id) => {
                if let Some(n) = self.symtable.get(&id) {
                    *n
                } else {
                    println!("*WARNING* variable {} is not defined, using 0", id);
                    0
                }
            }
        }
    }

    fn evaluate(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::Const(a) => {
                self.lookup(a)
            },
            Expression::Not(b) => {
                self.lookup(b)
            },
            Expression::And(a,b) => {
                self.lookup(a) & self.lookup(b)
            },
            Expression::Or(a,b) => {
                self.lookup(a) | self.lookup(b)
            }
        }
    }


    pub fn add_proc(&mut self, p: Procedure) {
        self.procedures.push(p);
    }

    pub fn show_proc(&self) {
        for i in 0..self.procedures.len() {
            println!("\nProcedure {}", i);
            self.procedures[i].show();
        }
    }

    pub fn show_symtable(&self) {
        println!("\nSymbol Table");
        println!("--------------------------------------");
        for (var, value) in self.symtable.iter() {
            println!(" {} = {}", var, value);
        }
        println!("--------------------------------------\n");
    }

    pub fn show_queues(&self) {
        println!("\nActive Queue");
        println!("--------------------------------------");
        for stmt in self.q_active.iter() {
            println!(" {}", stmt);
        }
        println!("--------------------------------------\n");
    }


}

