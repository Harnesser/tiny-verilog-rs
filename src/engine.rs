
use procedure::*;

use std::collections::VecDeque;
use std::collections::HashMap;

//use procedure::Value;

pub struct Engine {
    procedures : Vec<Procedure>,
    q_active : VecDeque<Statement>,
    q_nba    : VecDeque<Statement>, // nonblocking assignments
    symtable : HashMap<String, Value>,
}


impl Engine {

    pub fn new() -> Engine {
        Engine {
            symtable: HashMap::new(),
            procedures: vec![],
            q_active: VecDeque::new(),
            q_nba: VecDeque::new(),
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

            } else if self.q_nba.len() > 0 {
                println!("*INFO* Moving nonblocking assignments to active");
                while let Some(stmt) = self.q_nba.pop_back() {
                    self.q_active.push_front(stmt);
                }

            } else {
                println!("*INFO* Get events from procedures");
                let c_stmt = self.get_events();

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
                match id {
                    Operand::Identifier(i) => {
                        let val = self.evaluate(expr);
                        self.symtable.insert(i, val);
                    },
                    Operand::Literal(_) => {
                    },
                }
            },

            Statement::NonBlockingAssign{id, expr} => {
                match id {
                    Operand::Identifier(i) => {
                        let val = self.evaluate(expr);
                        let stmt = Statement::BlockingAssign{
                            id: Operand::Identifier(i),
                            expr: Expression::Const( Operand::Literal(val) ),
                        };
                        self.schedule_nba(stmt);
                    },
                    Operand::Literal(_) => {
                    },
                }
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


    fn schedule_nba(&mut self, stmt: Statement) {
        self.q_nba.push_front(stmt);
    }


    // pump each procedure until we hit a delay statement or the end
    fn get_events(&mut self) -> usize {
        let mut c_stmt:usize = 0;
        for p in &mut self.procedures {
            while let Some(stmt) = p.next() {
                match stmt {
                    Statement::Delay{dly} => {
                        println!("*INFO* Blocked on delay: {}", stmt)
                    },
                    _ => {
                        println!("*INFO* Loading: {}", stmt);
                        self.q_active.push_front(stmt);
                        c_stmt += 1;
                    }
                }
            }
        }
        c_stmt
    }

    //
    // Display stuff
    //
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

        println!("\nNonblocking Assignment Queue");
        println!("--------------------------------------");
        for stmt in self.q_nba.iter() {
            println!(" {}", stmt);
        }
        println!("--------------------------------------\n");
    }


}

