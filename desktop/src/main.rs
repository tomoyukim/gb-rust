use gb_core::*;

use std::time;
const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

fn main() {
    let time = time::Instant::now();
    let mut elapsed = 0;
    hello();
    loop {
        let e = time.elapsed().as_nanos();
        for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
            // TODO:

            elapsed += M_CYCLE_NANOS;
        }
    }
}
