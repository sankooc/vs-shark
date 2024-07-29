#[cfg(test)]
mod tests {

    #[test]
    fn testbasic() -> std::io::Result<()> {
        use crate::entry::load_data;
        use log::{error, info};
        use std::fs;
        use env_logger::{Builder, Target};
        env_logger::builder().is_test(true).try_init().unwrap();
        let fname = "../sandbox/demo.pcap";
        let data: Vec<u8> = fs::read(fname)?;
        let ctx = load_data(&data).unwrap();
        let frames = ctx.get_frames();
        // for f in frames.iter() {
        //     info!("inx:{} size:{}", f.summary.borrow().index, f.capture_size);
        //     let ff = f.eles.borrow();
        //     for e in ff.iter(){
        //         info!(" - {}", e.summary());
        //         let fields = e.get_fields();
        //         for field in fields.iter(){
        //             info!("   - {}", field.summary())
        //         }
        //     }
        // }
        Ok(())
    }
}
