#[cfg(test)]
mod tests {
    // use std::fs;
    // fn _parse(fname: &str) -> Instance {
    //     let data: Vec<u8> = fs::read(fname).unwrap();
    //     let _ctx = load_data(&data, Configuration::new(false)).unwrap();
    //     _ctx
    // }
    #[test]
    fn testbasic() -> std::io::Result<()> {
        // let fname = "../sandbox/demo.pcap";
        // let fname = "../sandbox/demo.pcapng";
        // let fname = "../../../pcaps/11.pcapng";

        // _parse(fname);

        // let data: Vec<u8> = fs::read(fname)?;
        // let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        // let _ctx = load_data(&data).unwrap();
        // let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        // let _frames = _ctx.get_frames();

        // for f in frames.iter() {
        //     match f.summary.borrow().protocol {
        //         Protocol::DNS => (),
        //         _ => continue,
        //     }
        //     info!(
        //         "inx:{} protocol: {:?} size:{}",
        //         f.summary.borrow().index,
        //         f.summary.borrow().protocol,
        //         f.capture_size
        //     );
        //     let ff = f.eles.borrow();
        //     for e in ff.iter() {
        //         info!("- {}", e.summary());
        //         let fields = e.get_fields();
        //         for field in fields.iter() {
        //             _dis(1, field);
        //         }
        //     }
        // }

        // let ct = _ctx.context();
        // let cons = ct.conversations();
        // for con in cons.values().into_iter() {
        //     info!("{} -{}", con.ep1.as_ref().borrow().host, con.ep2.as_ref().borrow().host, )
        // }
        // println!("finish cost {}", end -start);
        Ok(())
    }
}
