
use procedure::*;

#[allow(dead_code)]
pub fn build_proc1() -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };
    p.push( Statement::Delay{dly: 34} );
    p
}

#[allow(dead_code)]
pub fn build_proc2() -> Procedure {
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


// blocking assignments
#[allow(dead_code)]
pub fn build_proc3() -> Procedure {
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


// blocking assignments with a nonblocking
#[allow(dead_code)]
pub fn build_proc4() -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };

    // b1 = 5
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("b1".to_string()),
        expr: Expression::Const(
            Operand::Literal(5),
            )
        });

    // b2 = 10
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("b2".to_string()),
        expr: Expression::Const(
            Operand::Literal(10),
            )
        });

    // b1 <= 100
    p.push( Statement::NonBlockingAssign{
        id: Operand::Identifier("b1".to_string()),
        expr: Expression::Const(
            Operand::Literal(100),
            )
        });

    // b3 = b1 | b2 (should get 15)
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("b3".to_string()),
        expr: Expression::Or(
            Operand::Identifier("b1".to_string()),
            Operand::Identifier("b2".to_string()),
            )
        });

    p
}


// blocking assignments with a nonblocking, but behind a delay
#[allow(dead_code)]
pub fn build_proc5() -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };

    p.push( Statement::Delay{dly: 5} );

    // c1 = 5
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("c1".to_string()),
        expr: Expression::Const(
            Operand::Literal(5*256),
            )
        });

    // c2 = 10
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("c2".to_string()),
        expr: Expression::Const(
            Operand::Literal(10*256),
            )
        });

    // c1 <= 100
    p.push( Statement::NonBlockingAssign{
        id: Operand::Identifier("c1".to_string()),
        expr: Expression::Const(
            Operand::Literal(100*256),
            )
        });

    // c3 = c1 | c2 (should get 15*256)
    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("c3".to_string()),
        expr: Expression::Or(
            Operand::Identifier("c1".to_string()),
            Operand::Identifier("c2".to_string()),
            )
        });

    p
}


// always procedure on b3 and c3
#[allow(dead_code)]
pub fn build_proc6() -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Always,
            counter: 0,
            stmts: vec![],
    };


    p.push( Statement::AtChange{
        ids: vec![
            Operand::Identifier("b3".to_string()),
            Operand::Identifier("c3".to_string()),
        ],  
        });


    p.push( Statement::NonBlockingAssign{
        id: Operand::Identifier("at_b3_c3".to_string()),
        expr: Expression::Or(
            Operand::Identifier("b3".to_string()),
            Operand::Identifier("c3".to_string()),
            )
        });

    p

}


// build up a clock
#[allow(dead_code)]
pub fn build_clock(half_period: usize, cycles: usize) -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };

    p.push( Statement::BlockingAssign{
        id: Operand::Identifier("clk".to_string()),
        expr: Expression::Const(
            Operand::Literal(0),
            )
        });

    for _ in 0..cycles {
        p.push( Statement::Delay{dly: half_period} );

        // rise
        p.push( Statement::BlockingAssign{
            id: Operand::Identifier("clk".to_string()),
            expr: Expression::Const(
                Operand::Literal(1),
                )
            });

        p.push( Statement::Delay{dly: half_period} );

        // fall
        p.push( Statement::BlockingAssign{
            id: Operand::Identifier("clk".to_string()),
            expr: Expression::Const(
                Operand::Literal(0),
                )
            });
    }

    p
}

// build up a flip-flop
#[allow(dead_code)]
pub fn build_flop(ff_in: &str, ff_out: &str) -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Always,
            counter: 0,
            stmts: vec![],
    };

    p.push( Statement::AtChange{
        ids: vec![
            Operand::Identifier("clk".to_string()),
        ],  
        });

    p.push( Statement::BlockingAssign{
        id: Operand::Identifier(ff_out.to_string()),
        expr: Expression::Const(
            Operand::Identifier(ff_in.to_string()),
            )
        });

    p
}
