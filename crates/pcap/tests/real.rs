#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
    };

    use pcap::common::Instance;
    fn _parse(fname: &str) -> std::io::Result<Instance> {
        let mut ins = Instance::new();
        let path = Path::new(fname);
        if !path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
        }
        let file = File::open(fname)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::with_capacity(1024 * 1024);
        buffer.resize(1024 * 1024, 0); // 确保缓冲区有初始大小

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            ins.update(&buffer[..n]).unwrap();
        }
        Ok(ins)
    }
    #[test]
    fn testbasic() -> std::io::Result<()> {
        // let fname = "../../../pcaps/http.pcap";
        let fname = "../../../pcaps/http2.pcap";
        let _ins = _parse(fname)?;
        let ctx = _ins.get_context();
        println!("total frames {}", ctx.counter);
        Ok(())
    }
}
