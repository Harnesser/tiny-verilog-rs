
use procedure::*;
use timeheap::*;
use vcd::*;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;


//use procedure::Value;

pub struct Engine {
    procedures: Vec<Procedure>,
    q_active: VecDeque<Statement>,
    q_nba: VecDeque<Statement>, // nonblocking assignments
    symtable: HashMap<String, Value>,
    waiting: HashMap<Edge, HashSet<ProcId>>,
    timeheap: TimeHeap,
    time: Time,
    vars: Vec<String>, // list of vars in the design
    dumper: Option<VcdWriter>, // created later
}


impl Engine {

    pub fn new() -> Engine {
        Engine {
            symtable: HashMap::new(),
            procedures: vec![],
            waiting: HashMap::new(),
            q_active: VecDeque::new(),
            q_nba: VecDeque::new(),
            timeheap: TimeHeap::new(),
            time: 0,
            vars: vec![],
            dumper: None,
        }
    }

    pub fn init(&mut self) {
        println!("*INFO* Initialising timeheap");
        // fill the timeheap, set all trigger times to 0
        if self.procedures.is_empty() {
            panic!("*ERROR* no procedures to simulate");
        }

        for i in 0..self.procedures.len() {
            self.timeheap.push(i, 0);
        }

        println!("*INFO* Gathering variables used in the design");
        self.get_identifier_list();
        self.show_identifiers();

        println!("*INFO* Opening VCD file");
        self.dumper = VcdWriter::new("waves.vcd");
        if let Some(ref mut vcd) = self.dumper {
            vcd.write_header();
            vcd.declare_vars(&self.vars);
        }

    }


    fn get_identifier_list(&mut self) {
        self.vars.clear();
        for pid in &self.procedures {
            let proc_vars = pid.get_identifiers();
            for var in proc_vars {
                if !self.vars.contains(&var) {
                    self.vars.push(var);
                }
            }
        }
    }

    pub fn run(&mut self) {
        // simulation loop
        println!("\n*INFO* Starting simulation");
        let mut c_loop = 1;
        loop {
            println!("======================================================");
            println!("*INFO* Time: {}ns + {}", self.time, c_loop);
            //self.show_symtable();
            //self.show_queues();
            self.show_blocked_pids();

            if !self.q_active.is_empty() {
                println!("*INFO* Emptying active queue");
                while let Some(stmt) = self.q_active.pop_back() {
                    self.execute(stmt);
                }

            } else if !self.q_nba.is_empty() {
                println!("*INFO* Moving nonblocking assignments to active");
                while let Some(stmt) = self.q_nba.pop_back() {
                    self.q_active.push_front(stmt);
                }

            } else {
                println!("*INFO* Update VCD");
                if let Some(ref mut vcd) = self.dumper {
                    vcd.dump(self.time, &self.vars, &self.symtable);
                }

                println!("*INFO* Get events from procedures");
                c_loop = 0;
                let c_stmt = self.get_events();

                if c_stmt == 0 {
                    println!("*INFO* Event starved!");
                    break;
                }
            }
            c_loop += 1;
        }
        println!("======================================================");
        println!("\n*INFO* Finished at time {}", self.time);
        if let Some(ref mut vcd) = self.dumper {
            vcd.dump(self.time, &self.vars, &self.symtable);
        }
        self.show_blocked_pids();
        self.show_symtable();
    }


    fn execute(&mut self, stmt: Statement) {
        println!("*INFO* Executing: {}", stmt);
        match stmt {

            Statement::BlockingAssign{id, expr} => {
                match id {
                    Operand::Identifier(var) => {
                        let value = self.evaluate(expr);
                        self.update_variable(&var, value);
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

            _ => {
                println!("*WARNING* Statement not implemented: {}", stmt);
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

    fn update_time(&mut self, time: Time) {
        self.time = time;
        println!("*INFO* Time is now {}", self.time);
    }

    // pump each procedure until we hit a delay statement or the end
    fn get_events(&mut self) -> usize {
        let mut c_stmt:usize = 0;

        // get a list of the procedures activating in the next time step
        let (nexttime, proc_ids) = self.timeheap.activate();
        if let Some(time) = nexttime {
            self.update_time(time);
            println!("*INFO* Activating: {:?}", proc_ids);
        } else {
            println!("*INFO* Time starved");
            return 0;
        }

        // grab events from the active procedures and queue them up
        for pid in proc_ids {
            c_stmt += self.get_events_from_pid(pid);
        }
        c_stmt
    }


    fn get_events_from_pid(&mut self, pid: ProcId) -> usize {
        let mut c_stmt:usize = 0;
        let p = &mut self.procedures[pid];
        while let Some(stmt) = p.next_stmt() {
            match stmt {

                Statement::Delay{dly} => {
                    let trig_time = self.time + dly;
                    self.timeheap.push(pid, trig_time);
                    println!("*INFO* Procedure {} blocked on delay til: {}", 
                            pid, trig_time);
                    break;
                },

                Statement::AtChange{edges} => {
                    for edge in edges {
                        println!("*INFO* Process {} waits on {}", pid, edge);
                        let e = self.waiting.entry(edge).or_insert_with( HashSet::new );
                        e.insert(pid);
                    }
                    break;
                },

                _ => {
                    println!("*INFO* Loading: {}", stmt);
                    self.q_active.push_front(stmt);
                    c_stmt += 1;
                }
            }
        }
    c_stmt
    }


    // a value has changed, throw anythign sensive to this on
    // the active queue
    fn update_variable(&mut self, var: &str, value: Value) {

        // transitions
        let mut transitions: Vec<Edge> = vec![];

        // the 'something changed' trigger
        transitions.push( Edge::Any(var.to_string()) );

        // is there an edge trigger in here too?
        if let Some(old_value) = self.symtable.get(var) {
            if (*old_value == 0) & (value != 0) {
                // 0 -> something is a rising edge, triggers posedge blocks
                transitions.push( Edge::Rise(var.to_string()) );
            } else if (*old_value != 0) & (value == 0) {
                // something -> 0 is a falling edge and triggers negedge blocks
                transitions.push( Edge::Fall(var.to_string()) );
            }
        }

        println!("*INFO* Transition: {}", var);

        // update the variable
        self.symtable.insert(var.to_string(), value);

        // now trigger procedures sensitive to this var
        let mut pids_removed: Vec<ProcId> = vec![];
        for transition in transitions {
            if let Some(pid_set) = self.waiting.remove(&transition) {
                // activate the procedure that was waiting on a change
                for pid in pid_set {
                    println!("*INFO* pulling from {}", pid);
                    self.get_events_from_pid(pid);
                    pids_removed.push(pid);
                }
            }
        }
        //self.scrub_waiting_list(pids_removed);
    }

    #[allow(dead_code)]
    fn scrub_waiting_list(&mut self, pids: Vec<ProcId>) {
        for pid in pids {
            for values in self.waiting.values_mut() {
                values.remove(&pid);
            }
        }
    }

    //
    // Display stuff
    //
    #[allow(dead_code)]
    pub fn show_proc(&self) {
        for i in 0..self.procedures.len() {
            println!("\nProcedure {}", i);
            self.procedures[i].show();
        }
    }

    #[allow(dead_code)]
    pub fn show_symtable(&self) {
        println!("\nSymbol Table");
        println!("--------------------------------------");
        for (var, value) in &self.symtable {
            println!(" {} = {}", var, value);
        }
        println!("--------------------------------------\n");
    }

    #[allow(dead_code)]
    pub fn show_queues(&self) {
        println!("\nActive Queue");
        println!("--------------------------------------");
        for stmt in &self.q_active {
            println!(" {}", stmt);
        }
        println!("--------------------------------------\n");

        println!("\nNonblocking Assignment Queue");
        println!("--------------------------------------");
        for stmt in &self.q_nba {
            println!(" {}", stmt);
        }
        println!("--------------------------------------\n");
    }

    #[allow(dead_code)]
    pub fn show_blocked_pids(&self) {
        println!("\nBlocked Procedures");
        println!("--------------------------------------");
        for (var, value) in &self.waiting {
            println!(" {} -> {:?}", var, value);
        }
        println!("--------------------------------------\n");
    }


    #[allow(dead_code)]
    pub fn show_identifiers(&self) {
        println!("\nIdentifiers");
        println!("--------------------------------------");
        for var in &self.vars {
            println!(" {}", var);
        }
        println!("--------------------------------------\n");
    }


}

