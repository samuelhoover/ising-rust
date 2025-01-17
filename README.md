# 2d Ising model in Rust

Ising model on a 2,000 x 2,000 square lattice, in 1B steps, simulated *via*
Monte Carlo, implemented in Rust. Compiled executable completes simulation in
less than 6 seconds on M1 Pro processor.

<!-- markdownlint-disable MD033 -->
| Initial lattice | Final lattice |
|--|--|
| <img src="t_0.png" alt="Inital, random configuration lattice" width="400"> | <img src="t_1e9.png" alt="Final configuration lattice" width="400"> |

| Time evolution of relative spin count |
|--|
| <img src="count.png" alt="Time evolution of relative spin count" width="400"> |

To play around with the lattice, parameters, constants, *etc.*, modify
`src/main.rs` and call `cargo run` to run. Keep in mind that this simulation
will take considerably longer than the optimized executable version. To obtain
the optimized executable version, build the project using `cargo build -r` and
then call the executable (likely `/target/release/ising`).
