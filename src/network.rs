/// The Miso Network

const MAGIC_NUM_MAIN: u32 = 0xBBFACADE;
const MAGIC_NUM_TEST: u32 = 0x03251994;

pub enum Network {
    Mainnet,
    Testnet,
}

pub struct MagicError;

impl Network {
    pub fn magic(&self) -> u32 {
        match *self {
            Network::Mainnet => MAGIC_NUM_MAIN,
            Network::Testnet => MAGIC_NUM_TEST,
        }
    }

    pub fn from_magic(magic: u32) -> Result<Self, MagicError> {
        match magic {
            MAGIC_NUM_MAIN => Network::Mainnet,
            MAGIC_NUM_TEST => Network::Testnet,
        }
    }
}