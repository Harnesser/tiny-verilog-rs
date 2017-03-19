
run:
	cargo run | tee sim.log

debug:
	env RUST_BACKTRACE=1 cargo run

clippy:
	rustup run nightly cargo clippy


waves:
	gtkwave --dump waves.vcd --save plot.gtkw
