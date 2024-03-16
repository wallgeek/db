/*
    Disk deals with the File and 
    all the read and write operations
    to it.
*/
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::FileExt;

pub struct Disk(File);

impl Disk {
    pub fn new(file_path: &str) -> Self {
        let file_result = OpenOptions::new()
        .read(true)
        .append(true)
        .open(file_path);

        match file_result {
            Ok(file) => Self(file),
            Err(err) => panic!("Primary {:?}", err)
        }
    }

    pub fn put(&mut self, pos: usize, data: &Vec<u8>) {
        let _ = self.0.seek(SeekFrom::Start(pos as u64));
        let _ = self.0.write_at(data, pos as u64);
    }

    pub fn read(&mut self, pos: usize, len: usize) -> Vec<u8> {
        let _ = self.0.seek(SeekFrom::Start(pos as u64));

        let mut buffer: Vec<u8> = Vec::new();

        buffer.resize(len as usize, 0);

        let bytes_read = self.0.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            vec![]
        }else {
            buffer
        }
    }
}