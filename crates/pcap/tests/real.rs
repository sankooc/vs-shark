mod tc;
#[cfg(test)]
mod tests {
    use std::{
        fs::File, io::{BufReader, Read, Seek, SeekFrom}, mem, ops::Range, path::Path
    };
    use std::time::Instant;

    use pcap::{ common::{connection::TcpFlagField, enum_def::Protocol, Instance}};
    fn _parse(fname: &str) -> std::io::Result<Instance> {
        let mut ins = Instance::new();
        let path = Path::new(fname);
        if !path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
        }
        let file = File::open(fname)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::with_capacity(1024 * 1024 * 50);
        buffer.resize(1024 * 1024 * 50, 0);

        let mut total = 0;
        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            let start = Instant::now();
            ins.update(buffer[..n].to_vec()).unwrap();
            total += start.elapsed().as_millis();
        }
        println!("Elapsed: {} nanoseconds", total);
        
        Ok(ins)
    }
    
    fn _seek(fname: &str, range: Range<usize>) -> anyhow::Result<Vec<u8>> {
        let offset = range.start as u64;
        let size = range.end - range.start;
        let mut file = File::open(fname)?;
        file.seek(SeekFrom::Start(offset))?;
        let mut buffer = vec![0; size];
        file.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    #[test]
    fn basic() -> std::io::Result<()> { 
        // let fname = "../../../pcaps/11.pcapng";
        // let fname = "../../../pcaps/c1.pcap";
        // let fname = "../../../pcaps/demo.pcapng";
        // let fname = "../../../pcaps/demo.pcap";
        // let fname = "../../../pcaps/dns.pcapng";
        // let fname = "../../../pcaps/ftp.pcapng";
        // let fname = "../../../pcaps/http.pcapng";
        // let fname = "../../../pcaps/http.pcap";
        // let fname = "../../../pcaps/http2.pcap";
        // let fname = "../../../pcaps/http3.pcap";
        // let fname = "../../../pcaps/moden.pcapng";
        // let fname = "../../../pcaps/netbios.pcapng";
        // let fname = "../../../pcaps/pppoe.pcap";
        // let fname = "../../../pcaps/sip.pcap";
        // let fname = "../../../pcaps/slow.pcap";
        let fname = "../../../pcaps/big-2.pcap";
        let _ins = _parse(fname)?;
        // print!("--finish-");
        let ctx = _ins.get_context();
        println!("total frames {}", ctx.counter);
        println!("total conversations {}", _ins.connections_count());
        println!("etch cache {}", ctx.ethermap.len());
        println!("ipv6 cache {}", ctx.ipv6map.len());


        // {
        //     let json = _ins.frames_list_json(Criteria{ start: 0, size: 10})?;
        //     println!("{}", json);
        //     // return;
        // }
        // {
        //     let index = 13;
        //     let f = _ins.frame(index).unwrap();
        //     let range = f.range().unwrap();
        //     println!("range  {} - {}", range.start, range.end);
        //     let data = _seek(fname, range).unwrap();
        //     let json = _ins.select_frame(index, data).unwrap();
        //     crate::tc::print_fields(&json);
        // }
        // println!("json {}", json);
        Ok(())
    }

    // #[test]
    // fn pooltest() {
    //     let a1 = format!("{}-1k", "test");
    //     let a2 = format!("{}-1k", "test");
        
    //     println!("original: {:p} - {:p}", a1.as_ptr(), a2.as_ptr());
    //     println!("reference: {:p} - {:p}", (&a1).as_ptr(), (&a2).as_ptr());
    //     let c1 = intern(a1);
    //     let c2 = intern(a2);
    //     println!("cached: {:p} - {:p}", c1.as_ptr(), c2.as_ptr());
    // }

    #[test]
    fn pooltest2() {
        let ptoro = Protocol::SSL;
        println!("----------{}", ptoro);
        println!("----------{:?}", (ptoro as i32));
        println!("Size of Protocol: {} bytes", mem::size_of::<Protocol>()); 

        println!("ack => {}", TcpFlagField::from(0x0010).list_str());
        println!("push ack => {}", TcpFlagField::from(0x0018).list_str());
        println!("fin ack => {}", TcpFlagField::from(0x0011).list_str());
        println!("syn ack => {}", TcpFlagField::from(0x0012).list_str());
        println!("ret ack => {}", TcpFlagField::from(0x0014).list_str());
    }
}
