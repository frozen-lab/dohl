use caracal::Caracal;
use std::hint::black_box;

const ITER: usize = 1024 * 16;

fn main() {
    let mut acc = 0u64;

    let buf1 = vec![0xAAu8; 8];
    let buf2 = vec![0xBAu8; 16];
    let buf3 = vec![0xCAu8; 32];
    let buf4 = vec![0xDAu8; 64];
    let buf5 = vec![0xEAu8; 128];
    let buf6 = vec![0xFAu8; 256];
    let buf7 = vec![0xFFu8; 512];

    for _ in 0..ITER {
        acc ^= black_box(Caracal::hash64(&buf1));
        acc ^= black_box(Caracal::hash64(&buf2));
        acc ^= black_box(Caracal::hash64(&buf3));
        acc ^= black_box(Caracal::hash64(&buf4));
        acc ^= black_box(Caracal::hash64(&buf5));
        acc ^= black_box(Caracal::hash64(&buf6));
        acc ^= black_box(Caracal::hash64(&buf7));
    }

    println!("Accumulator: {:016x}", acc);
}
