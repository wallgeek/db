// This will be communicated with assembly line
// Assembly line will put 0 bytes value in the
// given pointer position of given length
use crate::flag::{ Flag, FlagType };

const TOTAL_BLOCK_BYTE: usize = 2;
const TOTAL_FLAG_BYTE: usize = 1;
const TOTAL_GOOD_LEN_BYTE: usize = 4;
const BLOCK_BYTE_POINTER: usize = 0;
const FLAG_BYTE_POINTER: usize = BLOCK_BYTE_POINTER + TOTAL_BLOCK_BYTE;
const GOOD_LEN_BYTE_POINTER: usize = FLAG_BYTE_POINTER + TOTAL_FLAG_BYTE;

#[derive(Clone, Copy)]
pub struct LabelInfo {
    pub block_count: u16,
    pub good_len: u32,
    pub flag_type: FlagType
}

pub struct Labeller;

impl Labeller {
    pub fn get_header_size() -> usize {
        TOTAL_BLOCK_BYTE + TOTAL_FLAG_BYTE + TOTAL_GOOD_LEN_BYTE
    }

    pub fn label(package: &mut [u8], label_info: LabelInfo) {
        let len = package.len();

        if len < Self::get_header_size() {
            panic!("Insufficient package length")
        }

        let block_count = label_info.block_count;
        let good_len = label_info.good_len;
        let flag_type = label_info.flag_type;
        
        let flag = Flag::get_value(flag_type);
        let block_bytes = block_count.to_le_bytes();
        let good_len_bytes = good_len.to_le_bytes();

        package[BLOCK_BYTE_POINTER] = block_bytes[0];
        package[BLOCK_BYTE_POINTER + 1] = block_bytes[1];

        package[FLAG_BYTE_POINTER] = flag;

        package[GOOD_LEN_BYTE_POINTER] = good_len_bytes[0];
        package[GOOD_LEN_BYTE_POINTER + 1] = good_len_bytes[1];
        package[GOOD_LEN_BYTE_POINTER + 2] = good_len_bytes[2];
        package[GOOD_LEN_BYTE_POINTER + 3] = good_len_bytes[3];
    }

    pub fn read(package: &[u8]) -> LabelInfo {
        if package.len() < Self::get_header_size() {
            println!("Insufficient package length")
        }

        let block_count = u16::from_le_bytes([
            package[BLOCK_BYTE_POINTER],
            package[BLOCK_BYTE_POINTER + 1]
        ]);

        let flag_type = Flag::get_type(package[FLAG_BYTE_POINTER]);

        let good_len = u32::from_le_bytes([
            package[GOOD_LEN_BYTE_POINTER],
            package[GOOD_LEN_BYTE_POINTER + 1],
            package[GOOD_LEN_BYTE_POINTER + 2],
            package[GOOD_LEN_BYTE_POINTER + 3]
        ]);

        LabelInfo {
            block_count,
            flag_type,
            good_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod label {
        use super::*;

        #[test]
        #[should_panic]
        fn insufficient_package_length(){
            Labeller::label(&mut Vec::<u8>::new(), LabelInfo {
                block_count: 0,
                flag_type: FlagType::Insert,
                good_len: 0
            })
        }
    }

    mod read {
        use super::*;

        #[test]
        #[should_panic]
        fn insufficient_package_length(){
            Labeller::read(&Vec::<u8>::new());
        }

        #[test]
        fn label_read(){
            let label_info = LabelInfo {
                block_count: 1,
                flag_type: FlagType::Insert,
                good_len: 1
            };

            let mut package: Vec<u8> = Vec::new();

            package.resize(Labeller::get_header_size(), 0);

            Labeller::label(&mut package, label_info);

            let read_label = Labeller::read(&package);

            assert_eq!(label_info.block_count, read_label.block_count);
            assert_eq!(label_info.flag_type, read_label.flag_type);
            assert_eq!(label_info.good_len, read_label.good_len);
        }
    }
}