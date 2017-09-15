use crypto::blake2s;
use hash::{H256, H512};

#[inline]
fn concat(a: &H256, b: &H256) -> H512 {
	let mut result = [0u8; 64];
	result[0..32].copy_from_slice(a);
	result[32..64].copy_from_slice(b);
	result
}

pub fn merkle_root(hashes: &[H256]) -> H256 {
	if hashes.len() == 1 {
		return hashes[0].clone();
	}

	let mut row = vec![];
	let mut i = 0;
	while i + 1 < hashes.len() {
		row.push(blake2s(&concat(&hashes[i], &hashes[i + 1])));
		i += 2
	}

	// duplicate the last element if len is not even
	if hashes.len() % 2 == 1 {
		let last = hashes[hashes.len() - 1];
		row.push(blake2s(&concat(&last, &last)));
	}

	merkle_root(&row)
}