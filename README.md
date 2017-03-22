Tiny Verilog Simulator
=========================

An implementation of a stripped-down Verilog simulation event loop.

Written in order to understand verilog scheduling semantics. In other
words, how a verilog simulator models the behaviour of a system of
parallel processes (AND gates, flip-flops etc.) on the sequential
processor of a computer? This explains the verbosity of the simulator
- it reports on everything that it's doing and plans to do.

*NOTE* This verilog simulator doesn't actually have a verilog codei
parser! Verilog procedural blocks (`initial` and `always`) are built up
programmatically before the `tiny-verilog-rs` simulator is invoked.

Supported:
 * `initial` and `always` blocks
 * Blocking assignments, e.g `a = 1`
 * Nonblocking assignments, e.g. `a <= 1`
 * Delay statements, e.g. `#3`
 * Wait statments, e.g. `@(a)`, `@(posedge a)` & `@(negedge a)`
 * And, Or & Not: `a = b | c`
 * VCD file output (single-bit variables)

Has:
 * Active event queue
 * Nonblocking assignment queue
 * Future event minheap

Ideas for future updates:
 * Continuous assignments
 * Branching and comparasons
 * Introduce "inactive" queue, e.g. `#0`
 * Switch to implement VHDL event loops, for comparason


Code Tour
==========
Implemented using the Rust programming language, compiler version 1.15. 
No external crates required.

 * `main.rs` - build a few procedures and invoke the simulator engine
 * `procedures.rs` - datastructure for an `initial` or `always` block,
  plus everything below ( statements, assignments, delays etc...)
 * `timeheap.rs` - future event min-heap
 * `engine.rs` - the main simulator event loop
 * `vcd.rs` - VCD waveform dumper
 * `test_procs.rs` - the art of verilog without verilog - routines to
  build up verilog procedures. Has things like `build_clock()` and 
  `build_bitstream()`.

To prove nonblockiness, change `NonBlockingAssign` in `build_flop()` to
`BlockingAssign` and compare waveforms of `ff3_out` before and after!

