use crate::def::{ Good, Package };
use crate::block::Block;
use crate::flag::FlagType;
use crate::labeller::{ LabelInfo, Labeller };

#[derive(Debug, PartialEq)]
pub enum Integrity {
    Inconsistent,
    Consistent(Good)
}

impl Integrity {
    pub fn unwrap(self) -> Good {
        match self {
            Self::Consistent(good) => good,
            Self::Inconsistent => panic!("Good is inconsistent")
        }
    }
}

struct Consistency;
impl Consistency {
    fn add_consistent_byte(good: &mut Good){
        good.push(u8::MAX);
    }

    fn check(good: &Good) -> bool {
        let len = good.len();
        let o_last = good.get(len - 1);

        if let Some(last) = o_last {
            if last == &u8::MAX {
                true
            }else {
                false
            }
        }else {
            false
        }
    }
}

pub struct AssemblyLine;

impl AssemblyLine {
    pub fn assemble(mut good: Good) -> Package {
        if good.len() == 0 {
            panic!("Good cannot be empty")
        }

        // add consistency byte
        Consistency::add_consistent_byte(&mut good);

        let good_len = good.len();
        let header_size = Labeller::get_header_size();

        // extend package to size of header + good
        let mut package: Package = Package::new();
        package.resize(header_size, 0);
        package.extend(good);

        // Caculate the block size to accomodate given package
        let block_count = Block::get_count(header_size + good_len);
        let block_size = Block::get_total_bytes(block_count);
        
        // need to extend package to block size
        package.resize(block_size, 0);
        
        let label_info = LabelInfo {
            block_count: block_count as u16,
            flag_type: FlagType::Insert,
            good_len: good_len as u32
        };

        Labeller::label(&mut package, label_info);

        package
    }

    pub fn dismantle(package: &[u8]) -> Integrity {
        let label_info = Labeller::read(package);
        let header_size = Labeller::get_header_size();

        let mut good = package[header_size..(header_size + (label_info.good_len as usize))].to_vec();

        let is_consistent = Consistency::check(&good);

        if is_consistent == true {
            good.pop();

            Integrity::Consistent(good)
        }else {
            Integrity::Inconsistent
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod assemble {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_good(){
            AssemblyLine::assemble(Vec::<u8>::new());
        }
    }

    mod dismantle {
        use super::*;
        use rand::Rng;

        #[test]
        #[should_panic]
        fn insufficient_package(){
            AssemblyLine::dismantle(&Vec::<u8>::new());
        }

        #[test]
        fn inconsistent() {
            let random_len = rand::thread_rng().gen_range(1..1000) as usize;
            let mut good: Vec<u8> = Vec::new();

            good.resize(random_len, 0);

            let mut package = AssemblyLine::assemble(good);

            // remove the consistent byte
            for index in Labeller::get_header_size()..package.len() {
                package[index] = 0;
            }

            let integrity = AssemblyLine::dismantle(&package);

            assert_eq!(integrity, Integrity::Inconsistent)
        }

        #[test]
        fn consistent(){
            let random_len = rand::thread_rng().gen_range(1..1000) as usize;
            let mut good: Vec<u8> = Vec::new();

            good.resize(random_len, 0);

            let package = AssemblyLine::assemble(good.clone());

            let integrity = AssemblyLine::dismantle(&package);
            
            if let Integrity::Consistent(dismantled_good) = integrity {
                assert_eq!(dismantled_good, good);
            }else {
                assert!(false)
            }
        }
    }
}