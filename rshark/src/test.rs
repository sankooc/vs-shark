use crate::entry::*;
use std::fs;

#[test]
fn testbasic() -> std::io::Result<()> {
    let data: Vec<u8> = fs::read("../sandbox/11.pcapng")?;
    let a: &[u8] = &data;
    load_data(a);
    Ok(())
}
