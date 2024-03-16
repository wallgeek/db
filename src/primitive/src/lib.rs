use scalar::Scalar;
// mod packet;
mod merchandise;
// pub use packet::Packet;
pub type Good = Vec<u8>;
pub struct Pair(pub String, pub Scalar);
pub use merchandise::Merchandise;

