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
//! | 8 B        | 4.405          |
//! | 16 B       | 4.148          |
//! | 32 B       | 4.933          |
//! | 64 B       | 7.450          |
//! | 128 B      | 12.467         |
//! | 256 B      | 23.410         |
//! | 512 B      | 45.397         |
//! | 1 KiB      | 95.524         |
//! | 2 KiB      | 182.261        |
//! | 4 KiB      | 364.616        |
//!
//! **Mean:** 74.46 ns/call  
//!
//! ## Perf Stats
//!
//! - ~3.74 IPC (Instructions per cycle)
//! - ~4.25 GHz effective core clock
//! - ~0.13% branch mispredicts (~2 branches per hash)
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
        let mut tmp = [0u8; 8];

        if buf.len() >= 4 {
            tmp[..4].copy_from_slice(&buf[..4]);
            let a_high = u32::from_le_bytes(tmp[..4].try_into().unwrap()) as u64;

            tmp[..4].copy_from_slice(&buf[buf.len() - 4..]);
            let a_low = u32::from_le_bytes(tmp[..4].try_into().unwrap()) as u64;

            a = (a_high << 32) | a_low;
        } else if !buf.is_empty() {
            a = (buf[0] as u64) << 16 | (buf[buf.len() / 2] as u64) << 8 | buf[buf.len() - 1] as u64;
        } else {
            a = 0;
        }

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
