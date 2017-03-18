
use procedure::*;

use std::collections::VecDeque;
use std::collections::HashMap;

type Value = usize;

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
    }


    fn execute(&mut self, stmt: Statement) {
        println!("*INFO* Executing: {}", stmt);
    }


    pub fn add_proc(&mut self, p: Procedure) {
        self.procedures.push(p);
    }

    pub fn show_proc(&self) {
        for i in 0..self.procedures.len() {
            self.procedures[i].show();
        }
    }

}

