#![allow(unused)]

use constants::DEFAULT_SEED;

mod constants {
    pub const PRIME64_1: u64 = 0x9E37_79B1_85EB_CA87;
    pub const PRIME64_2: u64 = 0xC2B2_AE3D_27D4_EB4F;
    pub const PRIME64_3: u64 = 0x1656_6791_9E37_79F9;
    pub const PRIME64_4: u64 = 0x85EB_CA77_C2B2_AE63;
    pub const PRIME64_5: u64 = 0x27D4_EB2F_1656_67C5;

    pub const RRMXMX_MUL: u64 = 0x9FB2_1C65_1E98_DF25;

    pub const SECRET_SIZE_MIN: usize = 0x88;
    pub const SECRET_DEFAULT_SIZE: usize = 0xC0;

    pub const XXH3_KSECRET: [u8; 0xC0] = [
        0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c, 0xde, 0xd4,
        0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f, 0xcb, 0x79, 0xe6, 0x4e,
        0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21, 0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43,
        0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c, 0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb,
        0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3, 0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19,
        0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8, 0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7,
        0xc7, 0x0b, 0x4f, 0x1d, 0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78,
        0x73, 0x64, 0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb,
        0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e, 0x2b, 0x16,
        0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce, 0x45, 0xcb, 0x3a, 0x8f,
        0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
    ];

    pub const ACC_INIT: [u64; 8] = [
        PRIME64_1, PRIME64_2, PRIME64_3, PRIME64_4, PRIME64_5, PRIME64_1, PRIME64_2, PRIME64_3,
    ];

    // avalanche
    const AVALANCHE_SHIFT1: u32 = 0x25;
    const AVALANCHE_SHIFT2: u32 = 0x20;

    // scramble
    const SCRAMBLE_SHIFT: u32 = 0x2F;

    // rrmxmx
    const RRMXMX_SHIFT1: u32 = 0x31;
    const RRMXMX_SHIFT2: u32 = 0x18;
    const RRMXMX_SHIFT3: u32 = 0x23;
    const RRMXMX_SHIFT4: u32 = 0x1C;

    pub const STRIPE_LANES: usize = 8; // 8 u64s
    pub const STRIPE_LEN: usize = 0x40; // bytes
    pub const BLOCK_LEN: usize = 0x400; // 16 stripes

    pub const DEFAULT_SEED: u64 = 0;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dohl;

impl Dohl {
    #[inline(always)]
    pub fn hash(input: &[u8]) -> u64 {
        let len = input.len();

        // very much unlikely
        if len == 0 {
            return Self::hash_len_0(constants::DEFAULT_SEED);
        }

        if len < 4 {
            return Self::hash_len_1_to_3(input, constants::DEFAULT_SEED);
        } else if len < 9 {
            return Self::hash_len_4_to_8(input, constants::DEFAULT_SEED);
        } else if len < 0x11 {
            return Self::hash_len_9_to_16(input, constants::DEFAULT_SEED);
        } else if len < 0x81 {
            return Self::hash_len_17_to_128(input, constants::DEFAULT_SEED);
        } else if len < 0xF1 {
            return Self::hash_len_129_to_240(input, constants::DEFAULT_SEED);
        }

        unsafe { core::hint::unreachable_unchecked() }
    }

    #[inline(always)]
    fn hash_len_0(seed: u64) -> u64 {
        let s1 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x38);
        let s2 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x40);
        let acc = seed ^ (s1 ^ s2);
        Self::avalanche(acc)
    }

    #[inline(always)]
    fn hash_len_1_to_3(input: &[u8], seed: u64) -> u64 {
        let len = input.len();
        let c1 = input[0] as u32;
        let c2 = input[len >> 1] as u32;
        let c3 = input[len - 1] as u32;

        let combined: u32 = (c1 << 0x10) | (c2 << 0x18) | (c3 << 0) | ((len as u32) << 8);

        // (secret[0..4] ^ secret[4..8]) + seed
        let s0 = Self::read_u32_le(&constants::XXH3_KSECRET, 0) as u64;
        let s1 = Self::read_u32_le(&constants::XXH3_KSECRET, 4) as u64;
        let bitflip = (s0 ^ s1).wrapping_add(seed);

        let keyed = (combined as u64) ^ bitflip;
        let mixed = keyed.wrapping_mul(constants::PRIME64_1);

        Self::avalanche(mixed)
    }

    #[inline(always)]
    fn hash_len_4_to_8(input: &[u8], mut seed: u64) -> u64 {
        let len = input.len();
        let seed32_swapped = Self::swap32(seed as u32) as u64;
        seed ^= seed32_swapped << 0x20;

        let input1 = Self::read_u32_le(input, 0) as u64;
        let input2 = Self::read_u32_le(input, len - 4) as u64;

        let s0 = Self::read_u64_le(&constants::XXH3_KSECRET, 8);
        let s1 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x10);
        let bitflip = (s0 ^ s1).wrapping_sub(seed);
        let combined = input2.wrapping_add(input1 << 32);

        let mut x = combined ^ bitflip;
        x ^= Self::rotl64(x, 0x31) ^ Self::rotl64(x, 0x18);
        x = x.wrapping_mul(constants::RRMXMX_MUL);
        x ^= (x >> 0x23).wrapping_add(len as u64);
        x = x.wrapping_mul(constants::RRMXMX_MUL);

        Self::xorshift64(x, 0x1C)
    }

    #[inline(always)]
    fn hash_len_9_to_16(input: &[u8], seed: u64) -> u64 {
        let len = input.len();
        let s1 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x18);
        let s2 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x20);
        let bitflip1 = (s1 ^ s2).wrapping_add(seed);

        let s3 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x28);
        let s4 = Self::read_u64_le(&constants::XXH3_KSECRET, 0x30);
        let bitflip2 = (s3 ^ s4).wrapping_sub(seed);

        let input_lo = Self::read_u64_le(input, 0) ^ bitflip1;
        let input_hi = Self::read_u64_le(input, len - 8) ^ bitflip2;

        let acc = (len as u64)
            .wrapping_add(Self::swap64(input_lo))
            .wrapping_add(input_hi)
            .wrapping_add(Self::mul128_fold64(input_lo, input_hi));

        Self::avalanche(acc)
    }

    #[inline]
    fn hash_len_17_to_128(input: &[u8], seed: u64) -> u64 {
        let len = input.len();
        let mut acc = (len as u64).wrapping_mul(constants::PRIME64_1);

        // Always mix front and back
        Self::mix16b(input, 0, 0, &mut acc);
        Self::mix16b(input, len - 0x10, 0x10, &mut acc);

        if len > 0x20 {
            Self::mix16b(input, 0x10, 0x20, &mut acc);
            Self::mix16b(input, len - 0x20, 0x30, &mut acc);

            if len > 0x40 {
                Self::mix16b(input, 0x20, 0x40, &mut acc);
                Self::mix16b(input, len - 0x30, 0x50, &mut acc);

                if len > 0x60 {
                    Self::mix16b(input, 0x30, 0x60, &mut acc);
                    Self::mix16b(input, len - 0x40, 0x70, &mut acc);
                }
            }
        }

        Self::avalanche(acc)
    }

    #[inline]
    fn hash_len_129_to_240(input: &[u8], seed: u64) -> u64 {
        let len = input.len();
        let mut acc = (len as u64).wrapping_mul(constants::PRIME64_1);

        // There are 16-byte chunks; we walk them with a stride of 16.
        // Number of 16B lanes we process is len / 16 (capped to match the secret size).
        let mut offset = 0;
        let mut secret_off = 0;

        // We process exactly 8 lanes from the front and 8 lanes from the back in the ref.
        // Here we approximate that pattern:
        while offset < 0x80 {
            Self::mix16b(input, offset, secret_off, &mut acc);
            Self::mix16b(input, offset + 0x10, secret_off + 0x10, &mut acc);

            let back = len - 0x10 - offset;
            Self::mix16b(input, back - 0x10, secret_off + 0x20, &mut acc);
            Self::mix16b(input, back, secret_off + 0x30, &mut acc);

            offset += 0x20;
            secret_off += 0x40;
        }

        Self::avalanche(acc)
    }

    #[inline]
    const fn rotl64(x: u64, r: u32) -> u64 {
        x.rotate_left(r)
    }

    #[inline]
    const fn xorshift64(v: u64, shift: u32) -> u64 {
        v ^ (v >> shift)
    }

    #[inline]
    const fn avalanche(mut h: u64) -> u64 {
        h ^= h >> 0x25;
        h = h.wrapping_mul(constants::PRIME64_3);
        h ^= h >> 0x20;
        h
    }

    #[inline]
    const fn mul128_fold64(a: u64, b: u64) -> u64 {
        let p = (a as u128).wrapping_mul(b as u128);
        (p as u64) ^ (p >> 0x40) as u64
    }

    #[inline]
    const fn swap32(x: u32) -> u32 {
        x.swap_bytes()
    }

    #[inline]
    const fn swap64(x: u64) -> u64 {
        x.swap_bytes()
    }

    #[inline]
    fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
        unsafe {
            let ptr = buf.as_ptr().add(offset) as *const u32;
            u32::from_le(std::ptr::read_unaligned(ptr))
        }
    }

    #[inline]
    fn read_u64_le(buf: &[u8], offset: usize) -> u64 {
        unsafe {
            let ptr = buf.as_ptr().add(offset) as *const u64;
            u64::from_le(std::ptr::read_unaligned(ptr))
        }
    }

    #[inline(always)]
    fn mix16b(input: &[u8], inp_off: usize, secret_off: usize, acc: &mut u64) {
        let lo = Self::read_u64_le(input, inp_off) ^ Self::read_u64_le(&constants::XXH3_KSECRET, secret_off);
        let hi = Self::read_u64_le(input, inp_off + 8) ^ Self::read_u64_le(&constants::XXH3_KSECRET, secret_off + 8);
        *acc = acc.wrapping_add(Self::mul128_fold64(lo, hi));
    }
}
