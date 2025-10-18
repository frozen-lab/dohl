use caracal::Caracal;
use std::time::Instant;

fn main() {
    let data = b"caracal";
    let iterations = 10_000_000;

    // Warm-up
    let _ = Caracal::hash64(data);

    let start = Instant::now();
    let mut acc = 0u64;

    for _ in 0..iterations {
        acc ^= Caracal::hash64(data);
    }

    let duration = start.elapsed();

    println!(
        "Hashed {} times in {:?} ({:.2} ns per call)",
        iterations,
        duration,
        duration.as_nanos() as f64 / iterations as f64
    );

    println!("Accumulator: {:016x}", acc);
}
