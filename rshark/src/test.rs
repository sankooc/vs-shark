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
        load_data(&data).unwrap();
        Ok(())
    }
}
