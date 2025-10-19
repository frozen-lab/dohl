//! Scalar impl (as baseline)
//!
//! ## Bench Results
//!
//! ```bash
//! taskset -c 0 perf stat cargo bench --bench thpt --profile bench
//! ```
//!
//! | Input Size | Time (ns/call) |
//! |------------|----------------|
//! | 8 B        | 2.765          |
//! | 16 B       | 2.768          |
//! | 32 B       | 4.375          |
//! | 64 B       | 6.850          |
//! | 128 B      | 12.756         |
//! | 256 B      | 24.627         |
//! | 512 B      | 46.501         |
//! | 1 KiB      | 94.886         |
//! | 2 KiB      | 177.650        |
//! | 4 KiB      | 352.293        |
//!
//! **Mean:** 72.547 ns/call (± 5.0)
//!
//! ## Perf Stats
//!
//! - Instruction throughput: ~3.81 IPC (Instructions per cycle)
//! - Effective core clock: ~4.22 GHz
//! - Branch mispredicts: ~0.13% (~2-3 branches per hash)
//!
const WYP: [u64; 4] = [
    0xa0761d6478bd642f,
    0xe7037ed1a0b428db,
    0x8ebc6af09c88c6e3,
    0x589965cc75374cc3,
];

const WYP_SEED: u64 = WYP[0];

#[inline(always)]
fn wymix(a: u64, b: u64) -> u64 {
    let r = (a as u128).wrapping_mul(b as u128);
    (r >> 64) as u64 ^ r as u64
}

#[inline(always)]
pub(crate) fn wyhash(buf: &[u8]) -> u64 {
    let mut seed = WYP_SEED;
    seed = wymix(seed, WYP[1] ^ buf.len() as u64);

    let mut a;
    let mut b;

    if buf.len() <= 16 {
        a = match buf.len() {
            0 => 0,
            1..=3 => {
                let mid = buf.len() / 2;
                ((buf[0] as u64) << 16) | ((buf[mid] as u64) << 8) | (buf[buf.len() - 1] as u64)
            }
            _ => {
                let a_high = u32::from_le_bytes(buf[..4].try_into().unwrap()) as u64;
                let a_low = u32::from_le_bytes(buf[buf.len() - 4..].try_into().unwrap()) as u64;
                (a_high << 32) | a_low
            }
        };

        b = seed ^ a.rotate_left(23);
        return wymix(a ^ WYP[1], b ^ WYP[3]);
    }

    let mut acc = seed;
    let mut d = buf;

    while d.len() > 16 {
        let mut tmp_a = [0u8; 8];
        let mut tmp_b = [0u8; 8];

        tmp_a.copy_from_slice(&d[..8]);
        tmp_b.copy_from_slice(&d[8..16]);

        a = u64::from_le_bytes(tmp_a);
        b = u64::from_le_bytes(tmp_b);

        acc = wymix(a ^ WYP[1], b ^ acc);
        d = &d[16..];
    }

    let mut tail = [0u8; 16];
    tail[..d.len()].copy_from_slice(d);

    a = u64::from_le_bytes([tail[0], tail[1], tail[2], tail[3], tail[4], tail[5], tail[6], tail[7]]);
    b = u64::from_le_bytes([
        tail[8], tail[9], tail[10], tail[11], tail[12], tail[13], tail[14], tail[15],
    ]);

    wymix(a ^ WYP[2], b ^ acc)
}

#[test]
fn test_if_wyhash_is_deterministic() {
    let data = b"caracal";

    let h1 = wyhash(data);
    let h2 = wyhash(data);

    println!("deterministic hash: {h1:016x}");
    assert_eq!(h1, h2, "wyhash must be deterministic for identical input");
}
