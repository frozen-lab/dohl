use caracal::Caracal;
use std::hint::black_box;

const ITER: usize = 1024 * 32 + 1;

fn main() {
    let mut acc = 0u64;

    for i in 0..ITER {
        let buf = vec![i as u8; 4096];
        acc ^= black_box(Caracal::hash64(&buf));
    }

    println!("Accumulator: {:016x}", acc);
}
