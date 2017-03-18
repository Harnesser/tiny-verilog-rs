
mod procedure;
mod engine;

use procedure::*;
use engine::Engine;

fn main() {
    println!("********************************************");
    println!("***     Tiny-Verilog-Simulator           ***");
    println!("***        (c) CrapCorp 2017             ***");
    println!("*** Patent Pending, All rights reserved  ***");
    println!("********************************************");

    println!("*INFO* Initialising");
    let mut eng = Engine::new();

    // build something to simulate
    println!("*INFO* Building design");
    eng.add_proc( build_proc1() );
    eng.add_proc( build_proc2() );
    eng.add_proc( build_proc3() );
    eng.show_proc();

    eng.run();
}


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


fn build_proc3() -> procedure::Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };

    // a1 = 31
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("a1".to_string()),
        expr: Expression::Const(
            Operand::Literal(31),
            )
        });

    // a2 = 32
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("a2".to_string()),
        expr: Expression::Const(
            Operand::Literal(32),
            )
        });

    // a3 = a1 | a2
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("a3".to_string()),
        expr: Expression::Or(
            Operand::Identifier("a1".to_string()),
            Operand::Identifier("a2".to_string()),
            )
        });

    // a1 = 0
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("a1".to_string()),
        expr: Expression::Const(
            Operand::Literal(0),
            )
        });

    p
}
