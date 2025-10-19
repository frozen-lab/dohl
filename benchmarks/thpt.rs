use caracal::Caracal;
use std::hint::black_box;
use std::time::Instant;

const ITER: usize = 1024 * 1024 + 1;

fn main() {
    let mut acc = 0u64;
    let mut total_ns = 0.0;

    let bufs = [
        vec![0xAAu8; 8],
        vec![0xBAu8; 16],
        vec![0xCAu8; 32],
        vec![0xDAu8; 64],
        vec![0xEAu8; 128],
        vec![0xFAu8; 256],
        vec![0xFEu8; 512],
        vec![0xABu8; 1024],
        vec![0xACu8; 2048],
        vec![0xADu8; 4096],
    ];

    for buf in &bufs {
        let start = Instant::now();

        for _ in 0..ITER {
            acc ^= black_box(Caracal::hash64(buf));
        }

        let dur = start.elapsed();
        let ns_per_call = dur.as_nanos() as f64 / ITER as f64;

        total_ns += ns_per_call;
        println!("{:4} bytes -> {:8.3} ns/call", buf.len(), ns_per_call);
    }

    println!(
        "\nMean time across all sizes: {:.3} ns/call",
        total_ns / bufs.len() as f64
    );
    println!("Accumulator: {:016x}", acc);
}
