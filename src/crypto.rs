use blake2::{Blake2s, Digest};
use hash::H256;

#[inline]
pub fn blake2s(input: &[u8]) -> H256 {
    let mut hasher = Blake2s::default();
    let data: &[u8] = input.clone();
    hasher.input(&data);

    let res = hasher.result();
    res.to_vec();
    let mut arr = [0u8;32];
    arr.clone_from_slice(&res);
    arr
}