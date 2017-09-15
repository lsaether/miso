
#[derive(Debug)]
struct Transaction {
    tx_header: TransactionHeader,
    signature: Signature,
    payload: Vec<u8>,
}

#[derive(Debug, Default)]
struct TransactionHeader {
    from: Vec<u8>,
    to: Vec<u8>,
    timestamp: u64,
    payload_hash: [u8;32],
    payload_length: u32,
    nonce: u32,
}

impl Transaction {
    fn new(from: &Vec<u8>, to: &Vec<u8>, payload: Vec<u8>) -> Transaction {
        let mut t = Transaction {
            tx_header: TransactionHeader::default(),
            signature: Signature::empty(),
            payload: payload,
        };
        t.tx_header.from = from;
        t.tx_header.to = to;
        t 
    }


}

impl TransactionHeader;