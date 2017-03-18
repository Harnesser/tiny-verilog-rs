mod procedure;

use std::collections::VecDeque;

use procedure::*;

fn build_proc1() -> procedure::Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };
    p.push( Statement::Delay{dly: 34} );
    p
}

fn build_proc2() -> procedure::Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };

    // c <= b | a
    p.push( Statement::NonBlockingAssign{
        id: Operand::Identifier("c".to_string()),
        expr: Expression::Or(
            Operand::Identifier("b".to_string()),
            Operand::Identifier("a".to_string()),
            )
        });

    // a = b & 3
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("a".to_string()),
        expr: Expression::And(
            Operand::Identifier("b".to_string()),
            Operand::Literal(3),
            )
        });

    p
}

fn main() {
    println!("Tiny-Verilog-Simulator");
    println!("(c) CrapCorp 2017");
    println!("Patent Pending, All rights reserved");

    println!("*INFO* Initialising");


    // queues
    let mut q_active: VecDeque<Statement> = VecDeque::new();
    let mut q_inactive: VecDeque<Statement> = VecDeque::new();

    let mut procedures: Vec<Procedure> = vec![];

    // build something to simulate
    println!("*INFO* Building design");
    procedures.push( build_proc1() );
    procedures.push( build_proc2() );
    for i in 0..procedures.len() {
        procedures[i].show();
    }


    // simulation loop
    println!("*INFO* Starting simulation");
    let mut c_loop = 0;
    loop {
        println!("\nLoop {}", c_loop);

        if q_active.len() > 0 {
            println!("*INFO* Emptying active queue");
            while q_active.len() > 0 {
                let stmt = q_active.pop_back();
                println!("*INFO* Executing: {:?}", stmt);
                // do stuff
            }
        } else if q_inactive.len() > 0 {
            println!("*INFO* Activiating inactive queue");
            while q_inactive.len() > 0 {
                let stmt =  q_inactive.pop_back().unwrap();
                println!("*INFO* Activating: {:?}", stmt);
                q_active.push_front(stmt);
            }
        } else {
            println!("*INFO* Get events from procedures");
            let mut c_stmt = 0;
            for p in &mut procedures {
                match p.next() {
                    Some(stmt) => {
                        println!("*INFO* Loading: {:?}", stmt);
                        q_inactive.push_front(stmt);
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

    println!("*INFO* Done");
}
