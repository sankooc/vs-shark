use std::{
    fs::File,
    io::{ErrorKind, Read, Seek, SeekFrom},
    ops::Range,
};

pub mod core;
pub struct FileBatchReader {
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
        // let mut buffer = Vec::with_capacity(self.block_size as usize);
        // buffer.resize(self.block_size as usize, 0);
        let mut buffer = vec![0; self.block_size as usize];
        let n = self.file.read(&mut buffer)?;
        if n == 0 {
            return Err(std::io::Error::new(ErrorKind::OutOfMemory, "overflow"));
        }
        let data = buffer[..n].to_vec();
        let extra = self.file_size.saturating_sub(n as u64);
        Ok((extra, data))
    }
}

pub fn file_seek(fname: &str, range: &Range<usize>) -> anyhow::Result<Vec<u8>> {
    let offset = range.start as u64;
    let size = range.end - range.start;
    let mut file = File::open(fname)?;
    file.seek(SeekFrom::Start(offset))?;
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

pub fn file_seeks(fname: &str, ranges: &[Range<usize>]) -> anyhow::Result<Vec<u8>> {
    if ranges.is_empty() {
        return Ok(vec![]);
    }
    let mut file = File::open(fname)?;
    let max = ranges.iter().map(|r| r.end - r.start).sum();

    let mut rs = Vec::with_capacity(max);
    for r in ranges {
        let start = r.start;
        let end: usize = r.end;
        file.seek(SeekFrom::Start(start as u64))?;
        let left = max - rs.len();
        let _size = end - start;
        let size = std::cmp::min(left, _size);
        let mut buffer = vec![0; size];
        file.read_exact(&mut buffer)?;
        rs.extend_from_slice(&buffer);
        if rs.len() >= max {
            break;
        }
    }

    Ok(rs)
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
