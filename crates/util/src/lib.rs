use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub struct FileBatchReader {
    // filename: String,
    file: File,
    file_size: u64,
    block_size: u64,
    count: u64,
}

impl FileBatchReader {
    pub fn new(filename: String, block_size: u64) -> Self {
        let file = File::open(&filename).unwrap();
        let file_size = file.metadata().unwrap().len();
        let count = file_size.div_ceil(block_size);
        Self {
            count,
            file,
            file_size,
            block_size,
        }
    }

    pub fn count(&self) -> u64 {
        self.count
    }
    pub fn read(&mut self) -> std::io::Result<(u64, Vec<u8>)> {
        let mut buffer = Vec::with_capacity(self.block_size as usize);
        buffer.resize(self.block_size as usize, 0);
        let n = self.file.read(&mut buffer)?;
        if n == 0 {
            return Err(std::io::Error::new(ErrorKind::OutOfMemory, "overflow"));
        }
        let data = buffer[..n].to_vec();
        let extra = self.file_size.saturating_sub(n as u64);
        Ok((extra, data))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
