#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
    };

    use pcap::{cache::intern, common::Instance};
    fn _parse(fname: &str) -> std::io::Result<Instance> {
        let mut ins = Instance::new();
        let path = Path::new(fname);
        if !path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
        }
        let file = File::open(fname)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::with_capacity(1024 * 1024);
        buffer.resize(1024 * 1024, 0);

        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            ins.update(buffer[..n].to_vec()).unwrap();
        }
        Ok(ins)
    }
    #[test]
    fn basic() -> std::io::Result<()> {
        let fname = "../../../pcaps/11.pcapng";
        // let fname = "../../../pcaps/http.pcap";
        // let fname = "../../../pcaps/http2.pcap";
        let _ins = _parse(fname)?;
        let ctx = _ins.get_context();
        println!("total frames {}", ctx.counter);
        let json = _ins.select_frame(0).unwrap();
        println!("json {}", json);
        Ok(())
    }

    #[test]
    fn pooltest() {
        let a1 = format!("{}-1k", "test");
        let a2 = format!("{}-1k", "test");
        
        println!("original: {:p} - {:p}", a1.as_ptr(), a2.as_ptr());
        println!("reference: {:p} - {:p}", (&a1).as_ptr(), (&a2).as_ptr());
        let c1 = intern(a1);
        let c2 = intern(a2);
        println!("cached: {:p} - {:p}", c1.as_ptr(), c2.as_ptr());
    }
}
