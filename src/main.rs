
mod procedure;
mod engine;
mod test_procs;
mod timeheap;
mod vcd;

use test_procs::*;
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
    eng.add_proc( build_clock(5, 1) );
    eng.add_proc( build_proc1() );
    eng.add_proc( build_proc2() );
    eng.add_proc( build_proc3() );
    eng.add_proc( build_proc4() );
    eng.add_proc( build_proc5() );
    eng.add_proc( build_proc6() );

    eng.show_proc();
    eng.init();

    eng.run();
}

