
use procedure::*;

use std::collections::VecDeque;

pub struct Engine {
    pub procedures : Vec<Procedure>,
    pub q_active   : VecDeque<Statement>,
}

impl Engine {

    pub fn new() -> Engine {
        Engine {
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
                while self.q_active.len() > 0 {
                    let stmt = self.q_active.pop_back();
                    println!("*INFO* Executing: {}", stmt.unwrap() );
                    // do stuff
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


}

