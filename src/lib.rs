mod scalar;

pub struct Caracal;

impl Caracal {
    pub fn hash64(buf: &[u8]) -> u64 {
        scalar::wyhash(buf)
    }
}
