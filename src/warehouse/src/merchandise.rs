type Good = Vec<u8>;

pub trait Merchandise: Clone {
    fn to_good(&self) -> Good;
    fn from_good(good: Good) -> Self; 
}