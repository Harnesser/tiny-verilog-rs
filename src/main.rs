mod procedure;

use procedure::*;

fn build_proc1() -> procedure::Procedure {
    Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    }
}

fn build_proc2() -> procedure::Procedure {
    Procedure {
            kind: ProcedureType::Initial,
            counter: 0,
            stmts: vec![],
    }
}

fn main() {
    println!("Hello, world!");
    let p1 = build_proc1();
    let p2 = build_proc2();

}
