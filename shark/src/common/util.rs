pub fn hexlize(data: &[u8]) -> String {
    data.iter().map(|f| format!("{:02x}", f)).collect::<String>()
}