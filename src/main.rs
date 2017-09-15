extern crate blake2;
extern crate crypto_mac;
extern crate ed25519_dalek;
extern crate generic_array;
extern crate rand;
extern crate time;

use blake2::{Blake2s, Blake2b, Digest}; // Blake2bs == 32 bytes ... Blake2b == 64 bytes
use crypto_mac::MacResult;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use generic_array::ArrayLength;
use rand::Rng;
use rand::OsRng;

fn blake2s_hash(input: Vec<u8>) -> [u8;32] {
    let mut hasher = Blake2s::default();
    let data: Vec<u8> = input.clone();
    hasher.input(&data);

    let res = hasher.result();
    res.to_vec();
    let mut arr = [0u8;32];
    arr.clone_from_slice(&res);
    arr
}

// fn blake2b_hash(input: Vec<u8>) -> [u8;64] {
//     let mut hasher = Blake2b::default();
//     let data: Vec<u8> = input.clone();
//     hasher.input(&data);

//     let res = hasher.result();
//     res.to_vec();
//     let mut arr = [0u8;64];
//     arr.clone_from_slice(&res);
//     arr
// }

fn timestamp() -> u64 {
    use time::get_time;

    let now = get_time();
    now.sec as u64
}

fn main() {
    // let mut cspring: OsRng = OsRng::new().unwrap();
    // let keypair: Keypair = Keypair::generate::<Blake2b>(&mut cspring);
    // // println!("{:?}", keypair);

    // let message: &[u8] = "This is a test of the tsunami alert system.".as_bytes();
    // let signature: Signature = keypair.sign::<Blake2b>(message);

    // let verified: bool = keypair.verify::<Blake2b>(message, &signature);
    // assert!(verified);

    // // println!("{:?}", signature);

    // let some_data = [1, 2, 3].to_vec();
    // let output2s = blake2s_hash(some_data.clone());
    // let output2b = blake2b_hash(some_data.clone());
    
    // let mut hasher = Blake2s::default();
    // let data: Vec<u8> = some_data.clone();
    // hasher.input(&data);

    // let output = hasher.result();

    // println!("{:?}\n\n{:?}\n\n", output.to_vec(), output2s);

    let time = timestamp();

    println!("{}", time);
}