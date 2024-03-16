/// DEFINITIONS:
/// Pointer = Position of a byte
/// Block = Series of bytes of fixed size
/// Package = Block count + flag + good size + good + filler
/// Good = Series of bytes
/// Flag = Represent the status of Package (Inserted/Deleted)
/// Filler = Extra null bytes

mod def;
mod flag;
mod disk;
mod block;
mod labeller;
mod logistics;
mod assembly_line;

use setup;
use estate::Farming;
use disk::Disk;
use flag::FlagType;
use block::Block;
use labeller::{LabelInfo, Labeller};
use assembly_line::{AssemblyLine, Integrity};
use def::{ GOOD_MAX_SIZE, CHUNK_SIZE, Package};
use fumble::Fumble;
pub use logistics::Logistics;
pub use logistics::Mode as LogisticsMode;
pub use def::{Good, Address};
pub use def::Item;

pub struct Godown {
    estate: Farming<Address>,
    disk: Disk
}

impl Godown {
    pub fn new(location: &str) -> Self {
        let header_size = Labeller::get_header_size();

        if CHUNK_SIZE <= header_size {
            panic!("Chunk size cannot be less than equal to header size")
        } 

        let full_path = setup::file(location);

        Self {
            estate: Farming::new(),
            disk: Disk::new(&full_path)
        }       
    }

    pub fn place(&mut self, good: Good) -> Result<Address, Fumble> {
        if good.len() > GOOD_MAX_SIZE {
            Err(Fumble::ScalarsCombinedSize)
        }else {
            // pack good
            let package = AssemblyLine::assemble(good);

            // get block count
            let label_info = Labeller::read(&package);
            
            // get a address
            let address = self.estate.allot(label_info.block_count as usize);

            // get location from address
            let location = self.estate.get_location(address).unwrap();

            // convert location to pointer position
            let pointer = Block::get_pointer(location);

            // put in disk
            self.disk.put(pointer, &package);

            Ok(address)
        }
    }

    pub fn remove(&mut self, address: Address){
        // get location
        let location = self.estate.get_location(address).unwrap();

        // get pointer
        let pointer = Block::get_pointer(location as usize);

        // need to create a emoty good
        let block_count = self.estate.get_size(address).unwrap();

        let package_len = Block::get_total_bytes(block_count);

        let mut package: Package = Vec::new();

        package.resize(package_len, 0);

        let label_info = LabelInfo {
            block_count: block_count as u16,
            flag_type: FlagType::Delete,
            good_len: 0
        };

        // lanel the block count
        Labeller::label(&mut package, label_info);

        self.disk.put(pointer, &package);

        self.estate.retain(address);
    }    

    pub fn replace(&mut self, address: Address, good: Good) -> Result<Address, Fumble> {
        let place_result = self.place(good);

        if place_result.is_err() {
            place_result
        }else {
            let replaced_address = place_result.ok().unwrap();
            self.remove(address);
            
            Ok(replaced_address)
        }
    }

    pub fn get(&mut self, address: Address) -> Good {
        let location = self.estate.get_location(address).unwrap();

        let pointer = Block::get_pointer(location);

        let size = self.estate.get_size(address).unwrap();

        let len = Block::get_total_bytes(size);
        let package = self.disk.read(pointer, len);
        
        AssemblyLine::dismantle(&package).unwrap()
    }

    pub fn transfer_chunk(&mut self, logistics: &mut Logistics) {
        let mut counter = 0;
        let header_size = Labeller::get_header_size();
        let pointer: usize = logistics.get_pointer();
        let mut block_number = logistics.get_block_number();
        let chunk = self.disk.read(pointer, CHUNK_SIZE);
        let chunk_len = chunk.len();
        
        // Note: Chunk len will always be equal CHUNK_SIZE
        // So we might be reading chunk with no or partial actual data
        // Or we may have cut down the last package if it all has actual data

        if chunk_len == 0 {
            logistics.set_has_done();
            return;
        }
        
        while counter + header_size < chunk_len {
            let label_info = Labeller::read(&chunk[counter..chunk_len]);
            let block_count = label_info.block_count;
            
            // Handle the case of no data. Block count will always be zero
            if block_count == 0 {
                logistics.set_has_done();
                break;
            }
        
            let package_len = Block::get_total_bytes(label_info.block_count as usize);
            
            if counter + package_len < chunk_len {
                let package: Package = chunk[counter..(counter + package_len)].to_vec();
                
                match logistics.get_mode() {
                    LogisticsMode::Uninitialize => {
                        let integrity = AssemblyLine::dismantle(&package);

                        match integrity {
                            Integrity::Consistent(good) => {
                                logistics.load_item(Item::WithoutAddress(good))
                            },
                            _ => {}
                        }
                    },

                    LogisticsMode::Initialize => {
                        let block_count_as_usize = block_count as usize;
                        let address: Address = self.estate.register(block_number, block_count_as_usize);
                        
                        if label_info.flag_type == FlagType::Delete {
                            self.estate.retain(address);
                        }else {
                           let integrity = AssemblyLine::dismantle(&package);
                            match integrity {
                                Integrity::Consistent(good) => {
                                    logistics.load_item(Item::WithAddress(address, good));
                                },
                                Integrity::Inconsistent => {
                                    self.remove(address);
                                }
                            }
                        }

                        block_number += block_count_as_usize;
                    }
                }
                
                counter += package_len;
            }else {
                break;
            }  
        }

        logistics.set_pointer(pointer + counter);
        logistics.set_block_number(block_number); 
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use std::fs;
    use uuid::Uuid;
    use rand::Rng;

    fn create_file_path() -> String{
        Uuid::new_v4().to_string()
    }

    fn remove_file(file_path: &str) {
        let folder_path = env::var("FOLDER_PATH").unwrap();
        let mut path = PathBuf::from(folder_path);
        path.push(file_path);
        let result = fs::remove_file(path);

        match result {
            Result::Err(err) => {
                println!("{:?}", err);
            },
            _ => {
                println!("Test file deleted successfully");
            }
        }
    }

    mod place {
        use super::*;

        // #[test]
        // fn successful(){
        //     let file_path = create_file_path();
        //     let mut godown = Godown::new(file_path.as_str());
        //     let random_len = rand::thread_rng().gen_range(1000..=5000);
        //     let mut good: Vec<u8> = Vec::new();

        //     for _ in 0..random_len {
        //         good.push(rand::thread_rng().gen_range(0..=255))
        //     }
            
        //     let address = godown.place(good.clone());
        //     let read_good = godown.get(address);
            
        //     remove_file(file_path.as_str());

        //     assert_eq!(good, read_good)
        // }
    }

    mod initialize {
        // use super::*;

        #[test]
        fn successful(){
            // let file_path = create_file_path();
            // let mut godown = Godown::new(file_path.as_str());
            // let good_number = rand::thread_rng().gen_range(10..=100);
            // let mut goods: Vec<Good> = Vec::new();
            // let mut addresses: Vec<Address> = Vec::new();

            // for _ in 0..good_number { 
            //     let random_len = rand::thread_rng().gen_range(10..=5000);
            //     let mut good: Good = Vec::new();

            //     for _ in 0..random_len {
            //         good.push(rand::thread_rng().gen_range(0..=255))
            //     }
                
            //     let address = godown.place(good.clone());
            //     goods.push(good);
            //     addresses.push(address);
            // }
            
            // let mut logistics = Logistics::new(LogisticsMode::Initialize);
            
            // let mut new_godown = Godown::new(&file_path.as_str());
            // new_godown.transfer_chunk(&mut logistics);
            
            // let logistics_good = logistics.unload_good();
            // let logistics_addresses = logistics.unload_address();

            // remove_file(file_path.as_str());

            // assert_eq!(goods.len(), logistics_good.len());
            // assert_eq!(addresses.len(), logistics_addresses.len());
            
            // for index in 0..goods.len() {
            //     assert_eq!(goods[index], logistics_good[index]);
            // }
        }
    }
}