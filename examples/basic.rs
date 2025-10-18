use caracal::Caracal;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    const ITERATIONS: usize = 10_000_000;

    let mut acc = 0u64;
    let mut buf = [0u8; 8];

    let start = Instant::now();

    for i in 0..ITERATIONS {
        buf[..8].copy_from_slice(&i.to_le_bytes());
        acc ^= black_box(Caracal::hash64(&buf));
    }

    let duration = start.elapsed();

    println!(
        "Hashed {} varying inputs in {:?} ({:.4} ns per call)",
        ITERATIONS,
        duration,
        duration.as_nanos() as f64 / ITERATIONS as f64
    );
    println!("Accumulator: {:016x}", acc);
}
