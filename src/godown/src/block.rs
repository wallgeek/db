/// A Block is a pre sized bytes. 
/// Every block holds package or a part of package.
/// Every block has a number attached to it according to its linear orderness in a Disk
/// A series of blocks may hold a package.
use crate::def::BLOCK_SIZE;

pub struct Block;

impl Block {
    pub fn get_count(package_len: usize) -> usize {
        if package_len == 0 {
            panic!("Package length cannot be zero")
        }

        let mut result = package_len / BLOCK_SIZE;
        
        if package_len % BLOCK_SIZE > 0 {
            result += 1;
        }
        
        result        
    }

    pub fn get_pointer(block_number: usize) -> usize {
        block_number * BLOCK_SIZE
    }

    pub fn get_total_bytes(block_count: usize) -> usize {
        block_count * BLOCK_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn get_count_equal_zero(){
        let bytes = 0;

        Block::get_count(bytes);
    }

    #[test]
    fn get_count_less_than_block_size(){
        let bytes = BLOCK_SIZE - 1;

        let count = Block::get_count(bytes);

        assert_eq!(count, 1)
    }

    #[test]
    fn get_count_equal_block_size(){
        let bytes = BLOCK_SIZE;

        let count = Block::get_count(bytes);

        assert_eq!(count, 1)
    }

    #[test]
    fn get_count_more_than_block_size(){
        let bytes = BLOCK_SIZE + 1;

        let count = Block::get_count(bytes);

        assert_eq!(count, 2)
    }
}