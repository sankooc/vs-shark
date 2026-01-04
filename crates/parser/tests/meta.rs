#[cfg(test)]
mod tests {
    use pcap::common::Instance;
    use util::core::LocalResource;



    #[test]
    fn read_metadata() -> anyhow::Result<()> {
        let fname = "./tests/metadata/base.pcapng";
        let batch_size = 1024 * 1024 * 4;
        let loader = LocalResource::new(fname.to_string());
        let mut ins = Instance::new(batch_size as usize, loader);

        let data = std::fs::read(fname)?;
        let _rs = ins.update(data)?;
        if let pcap::common::file::FileMetadata::PcapNg(pcapng) = ins.metadata() {
            assert!(pcapng.major == 1);
            assert!(pcapng.minor == 0);
            assert!(pcapng.interfaces.len() == 2);
            assert!(pcapng.capture.is_some());
        }
        
        Ok(())        
    }
}