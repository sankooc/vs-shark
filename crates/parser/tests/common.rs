mod ins {
    use pcap::common::{
        hex_num,
        quick_hash,
        // ResourceLoader,
        quick_string,
        quick_trim_num,
        range64,
        std_string,
        trim_data,
    };
    #[test]
    fn quick_string_basic() {
        let s = quick_string(b"hello");
        assert_eq!(s, "hello");
    }

    #[test]
    fn std_string_basic() {
        let s = std_string(b"world").unwrap();
        assert_eq!(s, "world");
    }

    #[test]
    fn quick_vs_std_same_ascii() {
        let data = b"The quick brown fox jumps over the lazy dog 12345";
        assert_eq!(quick_string(data), std_string(data).unwrap());
    }

    #[test]
    fn quick_string_empty() {
        let s = quick_string(b"");
        assert!(s.is_empty());
    }

    #[test]
    fn std_string_empty() {
        let s = std_string(b"").unwrap();
        assert!(s.is_empty());
    }

    #[test]
    fn quick_string_with_spaces() {
        let data = b"  leading and trailing  ";
        let s = quick_string(data);
        assert_eq!(s, "  leading and trailing  ");
    }

    #[test]
    fn std_string_long_input() {
        let data = vec![b'a'; 1024];
        let s = std_string(&data).unwrap();
        assert_eq!(s.len(), 1024);
        assert!(s.chars().all(|c| c == 'a'));
    }

    #[test]
    fn quick_string_long_input() {
        let data = vec![b'b'; 2048];
        let s = quick_string(&data);
        assert_eq!(s.len(), 2048);
        assert!(s.chars().all(|c| c == 'b'));
    }

    #[test]
    fn quick_string_trim_compare_manual() {
        let raw = b"  abc  ";
        let trimmed = trim_data(raw);
        assert_eq!(quick_string(trimmed), "abc");
    }

    #[test]
    fn std_string_trim_compare_manual() {
        let raw = b"\tabc\t";
        // trim_data likely only handles spaces; ensure original preserved
        let s = std_string(raw).unwrap();
        assert_eq!(s, "\tabc\t");
    }

    #[test]
    fn quick_string_hash_stability() {
        let s1 = quick_string(b"stable");
        let s2 = quick_string(b"stable");
        assert_eq!(quick_hash(&s1), quick_hash(&s2));
    }

    #[test]
    fn quick_string_numeric_interplay() {
        let raw = b"   98765  ";
        let num = quick_trim_num(raw).unwrap();
        assert_eq!(num, 98765);
        let s = quick_string(trim_data(raw));
        assert_eq!(s, "98765");
    }

    #[test]
    fn std_string_hex_interplay() {
        let raw = b"ff";
        let val = hex_num(raw).unwrap();
        assert_eq!(val, 255);
        let s = std_string(raw).unwrap();
        assert_eq!(s, "ff");
    }

    #[test]
    fn quick_string_large_mixed_content() {
        let mut data = Vec::new();
        data.extend_from_slice(b"HEADER:");
        data.extend((0..256).map(|i| b'a' + (i % 26) as u8));
        data.extend_from_slice(b":TAIL");
        let qs = quick_string(&data);
        let ss = std_string(&data).unwrap();
        assert_eq!(qs, ss);
        assert!(qs.starts_with("HEADER:"));
        assert!(qs.ends_with(":TAIL"));
    }

    #[test]
    fn std_string_unicode_safe_ascii_only() {
        let data = b"ASCII_ONLY";
        let s = std_string(data).unwrap();
        assert_eq!(s, "ASCII_ONLY");
        assert!(s.is_ascii());
    }

    #[test]
    fn quick_string_range64_helper_integration() {
        let r = 10..20;
        let r64 = range64(r.clone());
        let s = quick_string(format!("{}-{}", r64.start, r64.end).as_bytes());
        assert_eq!(s, "10-20");
    }

    #[test]
    fn quick_string_does_not_mutate_source() {
        let data = b"immutable".to_vec();
        let before = data.clone();
        let _ = quick_string(&data);
        assert_eq!(data, before);
    }

    #[test]
    fn std_string_does_not_mutate_source() {
        let data = b"immutable2".to_vec();
        let before = data.clone();
        let _ = std_string(&data);
        assert_eq!(data, before);
    }
}

mod load {

    use pcap::common::Instance;
    use util::core::LocalResource;

    fn pcapng_sample() -> Vec<u8> {
        fn pad4(len: usize) -> usize { (4 - (len % 4)) % 4 }
        fn opt(mut v: Vec<u8>, code: u16, data: &[u8]) -> Vec<u8> {
            let pad = pad4(data.len());
            v.extend_from_slice(&code.to_le_bytes());
            v.extend_from_slice(&(data.len() as u16).to_le_bytes());
            v.extend_from_slice(data);
            v.extend(std::iter::repeat(0u8).take(pad));
            v
        }
        fn end_opt(mut v: Vec<u8>) -> Vec<u8> {
            v.extend_from_slice(&0u16.to_le_bytes());
            v.extend_from_slice(&0u16.to_le_bytes());
            v
        }
        fn with_len(body: Vec<u8>, block_type: u32) -> Vec<u8> {
            let total_len = (8 + body.len() + 4) as u32;
            let mut out = Vec::with_capacity(total_len as usize);
            out.extend_from_slice(&block_type.to_le_bytes());
            out.extend_from_slice(&total_len.to_le_bytes());
            out.extend_from_slice(&body);
            out.extend_from_slice(&total_len.to_le_bytes());
            out
        }
    
        let mut file = Vec::new();
    
        // Section Header Block (0x0A0D0D0A) + meta options
        {
            let mut body = Vec::new();
            body.extend_from_slice(&0x4D3C2B1Au32.to_le_bytes()); // byte-order magic
            body.extend_from_slice(&1u16.to_le_bytes()); // major
            body.extend_from_slice(&0u16.to_le_bytes()); // minor
            body.extend_from_slice(&0xFFFFFFFFFFFFFFFFu64.to_le_bytes()); // section length unknown
            let mut opts = Vec::new();
            opts = opt(opts, 2, b"hw");
            opts = opt(opts, 3, b"linux");
            opts = opt(opts, 4, b"gen");
            opts = end_opt(opts);
            body.extend_from_slice(&opts);
            file.extend(with_len(body, 0x0A0D0D0A));
        }
    
        // Interface Description Block (0x00000001) + meta options
        {
            let mut body = Vec::new();
            body.extend_from_slice(&1u16.to_le_bytes()); // linktype (Ethernet)
            body.extend_from_slice(&0u16.to_le_bytes()); // reserved
            body.extend_from_slice(&65535u32.to_le_bytes()); // snaplen
            let mut opts = Vec::new();
            opts = opt(opts, 2, b"eth0");        // if_name
            opts = opt(opts, 3, b"primary nic"); // if_description
            opts = end_opt(opts);
            body.extend_from_slice(&opts);
            file.extend(with_len(body, 0x00000001));
        }
    
        // Name Resolution Block (0x00000004) + meta option
        {
            let mut body = Vec::new();
            // One IPv4 record: type=0x0001 length=4 addr + hostname record type=0x0002
            body.extend_from_slice(&0x0001u16.to_le_bytes());
            body.extend_from_slice(&4u16.to_le_bytes());
            body.extend_from_slice(&[192, 168, 0, 1]);
            body.extend(std::iter::repeat(0).take(pad4(4)));
            body.extend_from_slice(&0x0002u16.to_le_bytes());
            body.extend_from_slice(&9u16.to_le_bytes());
            body.extend_from_slice(b"router\x00"); // 9 bytes including null
            body.extend(std::iter::repeat(0).take(pad4(9)));
            // End of records
            body.extend_from_slice(&0u16.to_le_bytes());
            body.extend_from_slice(&0u16.to_le_bytes());
            // Block options
            let mut opts = Vec::new();
            opts = opt(opts, 1, b"nr"); // comment option
            opts = end_opt(opts);
            body.extend_from_slice(&opts);
            file.extend(with_len(body, 0x00000004));
        }
    
        // Interface Statistics Block (0x00000005) + meta options
        {
            let mut body = Vec::new();
            body.extend_from_slice(&0u32.to_le_bytes()); // interface id
            body.extend_from_slice(&0u32.to_le_bytes()); // ts_high
            body.extend_from_slice(&0u32.to_le_bytes()); // ts_low
            let mut opts = Vec::new();
            opts = opt(opts, 2, &[0,0,0,0,0,0,0,1]); // isb_starttime (8 bytes)
            opts = opt(opts, 3, &[0,0,0,0,0,0,0,2]); // isb_endtime
            opts = end_opt(opts);
            body.extend_from_slice(&opts);
            file.extend(with_len(body, 0x00000005));
        }
    
        // Enhanced Packet Block (0x00000006) ONLY data (no meta/options)
        {
            let mut body = Vec::new();
            body.extend_from_slice(&0u32.to_le_bytes()); // interface id
            body.extend_from_slice(&0u32.to_le_bytes()); // ts_high
            body.extend_from_slice(&1u32.to_le_bytes()); // ts_low
            let pkt: &[u8] = &[0x01,0x02,0x03,0x04,0x05,0x06];
            body.extend_from_slice(&(pkt.len() as u32).to_le_bytes()); // captured len
            body.extend_from_slice(&(pkt.len() as u32).to_le_bytes()); // original len
            body.extend_from_slice(pkt);
            body.extend(std::iter::repeat(0).take(pad4(pkt.len())));
            // no options
            file.extend(with_len(body, 0x00000006));
        }
    
        file
    }

    fn pcap_header() -> [u8; 24] {
        [
            0xd4, 0xc3, 0xb2, 0xa1, // magic (little-endian)
            0x02, 0x00, // major
            0x04, 0x00, // minor
            0x00, 0x00, 0x00, 0x00, // thiszone
            0x00, 0x00, 0x00, 0x00, // sigfigs
            0xff, 0xff, 0x00, 0x00, // snaplen
            0x01, 0x00, 0x00, 0x00, // network (Ethernet)
        ]
    }

    fn pcap_packet(payload_len: usize, pattern: u8) -> Vec<u8> {
        let mut v = Vec::with_capacity(16 + payload_len);
        let ts_sec: u32 = 1;
        let ts_usec: u32 = 2;
        let incl_len: u32 = payload_len as u32;
        let orig_len: u32 = payload_len as u32;
        v.extend_from_slice(&ts_sec.to_le_bytes());
        v.extend_from_slice(&ts_usec.to_le_bytes());
        v.extend_from_slice(&incl_len.to_le_bytes());
        v.extend_from_slice(&orig_len.to_le_bytes());
        v.extend(std::iter::repeat(pattern).take(payload_len));
        v
    }

    fn pcap_with_packets(n: usize, payload_len: usize) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&pcap_header());
        for i in 0..n {
            v.extend(pcap_packet(payload_len, 0xA0u8.wrapping_add(i as u8)));
        }
        v
    }
    #[test]
    fn pcap_success() {
        let batch_size = 1024 * 1024 * 4;
        let loader = LocalResource::new("".to_string());
        let mut ins = Instance::new(batch_size as usize, loader);

        let data1 = pcap_with_packets(1, 60);
        let data2 = pcap_packet(80, 0xCC);

        if let Ok(rs) = ins.update(data1) {
            assert_eq!(rs.total, 100);
            assert_eq!(rs.count, 1);
        } else {
            assert!(false, "Failed to update with data");
        }
        if let Ok(rs) = ins.update(data2) {
            assert_eq!(rs.total, 196);
            assert_eq!(rs.count, 2);
        } else {
            assert!(false, "Failed to update with data");
        }
    }
    #[test]
    fn pcapng_success() {
        let batch_size = 1024 * 1024 * 4;
        let loader = LocalResource::new("".to_string());
        let mut ins = Instance::new(batch_size as usize, loader);

        let data1 = pcapng_sample();

        if let Ok(rs) = ins.update(data1) {
            assert_eq!(rs.total, 250);
            assert_eq!(rs.count, 1);
        } else {
            assert!(false, "Failed to update with data");
        }
    }
}


#[cfg(test)]
mod instance {
    use std::ops::Range;
    use pcap::common::{Frame, Instance, ResourceLoader};
    use anyhow::Result;

    // -----------------------------
    // Mock ResourceLoader
    // -----------------------------
    #[derive(Default)]
    struct MockLoader {
        pub _loaded_ranges: Vec<Range<usize>>,
        pub data: Vec<u8>,
    }

    impl ResourceLoader for MockLoader {
        fn load(&self, range: &Range<usize>) -> Result<Vec<u8>> {
            Ok(self.data[range.clone()].to_vec())
        }

        fn loads(&self, ranges: &[Range<usize>]) -> Result<Vec<u8>> {
            let mut rs = vec![];
            for r in ranges {
                rs.extend_from_slice(&self.data[r.clone()]);
            }
            Ok(rs)
        }
    }

    fn new_instance_with_mock() -> Instance<MockLoader> {
        Instance::new(1024, MockLoader::default())
    }

    #[test]
    fn test_context() {
        let instance = new_instance_with_mock();
        let ctx = instance.context();
        assert_eq!(ctx.list.len(), 0);
        assert_eq!(ctx.counter, 0);
    }

    #[test]
    fn test_destroy() {
        let mut instance = new_instance_with_mock();

        instance.ctx.list.push(Frame::default());

        let ok = instance.destroy();
        assert!(ok);

        assert_eq!(instance.context().list.len(), 0);
    }

    #[test]
    fn test_update_basic() {
        let mut instance = new_instance_with_mock();

        let rs = instance.update(vec![1, 2, 3, 4]);
        assert!(rs.is_err());
    }

    #[test]
    fn test_update_slice_basic() {
        let mut instance = new_instance_with_mock();

        let rs = instance.update_slice(&[1, 2, 3, 4]);
        assert!(rs.is_err());
    }

    #[test]
    fn test_stat_ip4() {
        let instance = new_instance_with_mock();
        let v = instance.stat_ip4();
        assert!(v.is_empty());
    }

}
