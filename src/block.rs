use crypto::blake2s;

#[derive(Debug, Default)]
struct TransactionList;

/// The Block struct represents the compact data representing one unit of the chain.
struct Block {
    block_header: BlockHeader,
    transactions: TransactionList,
    // TODO omners (?)
}

/// The BlockHeader contains all of the relevant data to verify the chain, but no records of 
/// transactions. This is useful for light clients that only want to keep up with the chain and
/// verify blocks but have no concern for keeping a record of the full transactions.
#[derive(Debug, Default)]
struct BlockHeader {
    coinbase: [u8;32],
    prev_block_hash: [u8;32],
    block_hash: [u8;32],
    timestamp: u64,
    nonce: u32,
}

impl Block {
    fn generate(header: BlockHeader, txList: TransactionList) -> Block {
        Block{
            block_header: header,
            transactions: txList;
        }
    }

    fn hash(&Self) {
        // TODO encode the fields and hash them like below.
        let mut res = [0u8; 160];


    }

    fn hex_hash(&Self) -> //TODO implement hex_encode that takes [u8;160] {
        hex_encode(self.hash())
    }



}

impl BlockHeader {
    fn from_prev(prev: &Block) -> BlockHeader {
        let mut result = [0u8; 128];
        result[0..32].copy_from_slice(prev.prev_block_hash);
        result[32..64].copy_from_slice(prev.block_hash);
        result[64..96].copy_from_slice(blake2s(prev.timestamp));
        result[96..128].copy_from_slice(blake2s(prev.nonce));

        let mut block_header = BlockHeader::default();
        block_header.prev_block_hash = blake2s(result);
        block_header.timestamp = timestamp(); // TODO implement timestamp macro
        block_header.nonce = prev.nonce + 1;

        block_header
    }
}

/// TODO: implement Blake2s merkle rooting

#[inline]
fn timestamp() -> u64 {
    use time::get_time;

    let now = get_time();
    now.sec as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
