
use procedure::*;

#[allow(dead_code)]
pub fn build_delay(dly: usize) -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };
    p.push( Statement::Delay{dly: dly} );
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
        edges: vec![
            Edge::Rise("clk".to_string()),
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

// build up a flip-flop with a reset - need 'if' statement
#[allow(dead_code)]
pub fn build_flop_with_resetb(ff_in: &str, ff_out: &str) -> Procedure {
    let mut p = Procedure {
            kind: ProcedureType::Always,
            counter: 0,
            stmts: vec![],
    };

    p.push( Statement::AtChange{
        edges: vec![
            Edge::Rise("clk".to_string()),
            Edge::Fall("resetb".to_string()),
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


// build up a bitstream
#[allow(dead_code)]
pub fn build_bitstream(wire: &str, data:usize, len:usize, period: usize) -> Procedure {

    let mut p = Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    };

    assert!(len <= 32);
    assert!(period <= 50);

    for i in 0..len {
        let data = ( data >> i ) & 1;
        p.push( Statement::BlockingAssign{
            id: Operand::Identifier(wire.to_string()),
            expr: Expression::Const(Operand::Literal(data)),
            });

        p.push( Statement::Delay{dly: period} );
    }

    p
}

// build up a bitstream
#[allow(dead_code)]
pub fn build_and(y: &str, a: &str, b: &str) -> Procedure {

    let mut p = Procedure {
            kind: ProcedureType::Always,
            counter: 0,
            stmts: vec![],
    };

    p.push( Statement::AtChange{
        edges: vec![
            Edge::Any(a.to_string()),
            Edge::Any(b.to_string()),
        ],  
        });


    p.push( Statement::NonBlockingAssign{
        id: Operand::Identifier(y.to_string()),
        expr: Expression::And(
            Operand::Identifier(a.to_string()),
            Operand::Identifier(b.to_string()),
            )
        });

    p

}

// build up a inverter
#[allow(dead_code)]
pub fn build_inverter(y: &str, a: &str) -> Procedure {

    let mut p = Procedure {
            kind: ProcedureType::Always,
            counter: 0,
            stmts: vec![],
    };

    p.push( Statement::AtChange{
        edges: vec![
            Edge::Any(a.to_string()),
        ],  
        });


    p.push( Statement::NonBlockingAssign{
        id: Operand::Identifier(y.to_string()),
        expr: Expression::Not(
            Operand::Identifier(a.to_string()),
            )
        });

    p

}

