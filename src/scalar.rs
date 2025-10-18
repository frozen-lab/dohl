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

pub(crate) fn wyhash(buf: &[u8]) -> u64 {
    let mut seed = WYP_SEED;
    seed = wymix(seed, WYP[1] ^ buf.len() as u64);

    let mut a;
    let mut b;

    if buf.len() <= 16 {
        if buf.len() >= 4 {
            a = (u32::from_le_bytes(buf[..4].try_into().unwrap()) as u64) << 32
                | u32::from_le_bytes(buf[buf.len() - 4..].try_into().unwrap()) as u64;
        } else if !buf.is_empty() {
            a = (buf[0] as u64) << 16 | (buf[buf.len() / 2] as u64) << 8 | *buf.last().unwrap() as u64;
        } else {
            a = 0;
        }

        b = seed ^ a.rotate_left(23);
        return wymix(a ^ WYP[1], b ^ WYP[3]);
    }

    let mut acc = seed;
    let mut d = buf;

    while d.len() > 16 {
        a = u64::from_le_bytes(d[..8].try_into().unwrap());
        b = u64::from_le_bytes(d[8..16].try_into().unwrap());

        acc = wymix(a ^ WYP[1], b ^ acc);
        d = &d[16..];
    }

    let mut tail = [0u8; 16];
    tail[..d.len()].copy_from_slice(d);

    a = u64::from_le_bytes(tail[..8].try_into().unwrap());
    b = u64::from_le_bytes(tail[8..].try_into().unwrap());

    wymix(a ^ WYP[2], b ^ acc)
}

#[test]
fn wyhash_deterministic() {
    let data = b"caracal";

    let h1 = wyhash(data);
    let h2 = wyhash(data);

    println!("deterministic hash: {h1:016x}");
    assert_eq!(h1, h2, "wyhash must be deterministic for identical input");
}
