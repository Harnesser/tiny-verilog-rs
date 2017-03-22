
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
    eng.add_proc( build_clock(5, 100) );
    eng.add_proc( build_bitstream("a", 0xF0F0F0F0, 32, 7, 0) );
    eng.add_proc( build_bitstream("b", 0x34AE4210, 32, 11, 0) );
    eng.add_proc( build_bitstream("c", 0x834ead51, 32, 13, 200) );
    eng.add_proc( build_and("y", "a", "b") );
    eng.add_proc( build_flop("y", "ff1_out"));
    eng.add_proc( build_flop("ff1_out", "ff2_out"));
    eng.add_proc( build_flop("ff2_out", "ff3_out"));
    eng.add_proc( build_flop("ff_div_inv", "ff_div"));
    eng.add_proc( build_inverter("ff_div_inv", "ff_div") );



    eng.show_proc();
    eng.init();

    eng.run();
}

