pub const GOOD_MAX_SIZE: usize = 32 * 1024; // 32 kb
pub const BLOCK_SIZE: usize = 256; // 256 byte
pub const CHUNK_SIZE: usize = GOOD_MAX_SIZE * 2; // 64 kb
pub type Address = u64;
pub type Package = Vec<u8>;
pub type Good = Vec<u8>;

pub enum Item {
    WithAddress(Address, Good),
    WithoutAddress(Good)
}