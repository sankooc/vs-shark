#[cfg(test)]
mod tests {

    //
    // -------------------- IO --------------------
    //

    use std::net::Ipv4Addr;

    use pcap::common::io::*;

    #[test]
    fn test_io_read16() {
        let data = [0x12, 0x34];
        assert_eq!(IO::read16(&data, true).unwrap(), 0x1234);
        assert_eq!(IO::read16(&data, false).unwrap(), u16::from_le_bytes(data));
    }

    #[test]
    fn test_io_read32() {
        let data = [1, 2, 3, 4];
        assert_eq!(IO::read32(&data, true).unwrap(), 0x01020304);
    }

    #[test]
    fn test_io_read64() {
        let data = [1u8,2,3,4,5,6,7,8];
        assert_eq!(IO::_read64(&data, true).unwrap(), 0x0102030405060708);
    }

    //
    // -------------------- DataSource --------------------
    //

    #[test]
    fn test_datasource_create_and_slice() {
        let ds = DataSource::create(vec![1,2,3,4], 10..14);

        assert_eq!(ds.range(), 10..14);
        assert_eq!(ds.slice(10..12).unwrap(), &[1,2]);
    }

    #[test]
    fn test_datasource_update_and_trim() {
        let mut ds = DataSource::create(vec![1,2,3,4], 0..4);
        ds.update(vec![5,6]);

        assert_eq!(ds.range(), 0..6);
        assert_eq!(ds.data, vec![1,2,3,4,5,6]);

        ds.trim(3).unwrap();
        assert_eq!(ds.range(), 3..6);
        assert_eq!(ds.data, vec![4,5,6]);
    }

    #[test]
    fn test_datasource_destroy() {
        let mut ds = DataSource::create(vec![1,2,3], 5..8);
        ds.destroy();
        assert_eq!(ds.range(), 0..0);
        assert_eq!(ds.data.len(), 0);
    }

    //
    // -------------------- Reader basics --------------------
    //

    #[test]
    fn test_reader_basic() {
        let ds = DataSource::create(vec![10,20,30,40], 100..104);
        let mut r = Reader::new(&ds);

        assert_eq!(r.left(), 4);
        assert_eq!(r.read8().unwrap(), 10);
        assert_eq!(r.read8().unwrap(), 20);
        assert_eq!(r.left(), 2);
    }

    #[test]
    fn test_reader_slice_as_reader() {
        let ds = DataSource::create(vec![1,2,3,4,5], 0..5);
        let mut r = Reader::new(&ds);

        let sub = r.slice_as_reader(3).unwrap();
        assert_eq!(sub.preview(3).unwrap(), &[1,2,3]);
    }

    #[test]
    fn test_reader_read16_32_64() {
        let ds = DataSource::create(vec![0x01,0x02, 0x03,0x04, 5,6,7,8], 0..8);
        let mut r = Reader::new(&ds);

        assert_eq!(r.read16(true).unwrap(), 0x0102);
        assert_eq!(r.read16(true).unwrap(), 0x0304);
        assert_eq!(r.read32(false).unwrap(), u32::from_le_bytes([5,6,7,8]));
    }

    #[test]
    fn test_reader_read_string() {
        let ds = DataSource::create(b"hello world".to_vec(), 0..11);
        let mut r = Reader::new(&ds);

        assert_eq!(r.read_string(5).unwrap(), "hello");
    }

    #[test]
    fn test_reader_read_mac_ip() {
        let mac_data = vec![1,2,3,4,5,6, 192,168,1,1, 0,0,0,0,0,0];
        let ds = DataSource::create(mac_data, 0..16);
        let mut r = Reader::new(&ds);

        assert_eq!(r.read_mac().unwrap().data, [1,2,3,4,5,6]);
        assert_eq!(r.read_ip4().unwrap(), Ipv4Addr::new(192,168,1,1));
    }

    #[test]
    fn test_reader_hash() {
        let ds = DataSource::create(vec![9,9,9], 0..3);
        let r = Reader::new(&ds);

        let h1 = r.hash();
        let h2 = r.hash();

        assert_eq!(h1, h2); // deterministic
    }

    #[test]
    fn test_reader_search_enter() {
        let ds = DataSource::create(b"abc\r\ndef".to_vec(), 0..8);
        let mut r = Reader::new(&ds);

        assert_eq!(r.search_enter(8), Some(3));
    }

    #[test]
    fn test_reader_extract_left() {
        let ds = DataSource::create(vec![1,2,3,4], 0..4);
        let mut r = Reader::new(&ds);

        r.forward(2);
        let left = r.extract_left().unwrap();

        assert_eq!(left.data, vec![3,4]);
        assert_eq!(left.range, 2..4);
    }

    //
    // -------------------- Utilities --------------------
    //

    #[test]
    fn test_find_crlf() {
        let pos = find_crlf(b"hello\r\nworld");
        assert_eq!(pos, Some(5));

        let none = find_crlf(b"hello\nworld");
        assert_eq!(none, None);
    }

    #[test]
    fn test_read_mac_format() {
        let s = read_mac(&[1,2,3,4,5,6]);
        assert_eq!(s, "01:02:03:04:05:06");
    }

    #[test]
    fn test_macaddress_display() {
        let mac = MacAddress::from([1,2,3,4,5,6]);
        assert_eq!(format!("{}", mac), "01:02:03:04:05:06");
    }

        //
    // -------------------- Reader::back --------------------
    //
    #[test]
    fn test_reader_back() {
        let ds = DataSource::create(vec![10,20,30,40], 0..4);
        let mut r = Reader::new(&ds);

        r.forward(3); 
        assert_eq!(r.cursor, 3);

        assert!(r.back(2));
        assert_eq!(r.cursor, 1);

        assert!(!r.back(5)); // out of range
        assert_eq!(r.cursor, 1);
    }

    //
    // -------------------- Reader::slice mv=true/false --------------------
    //
    #[test]
    fn test_reader_slice_mv_true() {
        let ds = DataSource::create(vec![1,2,3,4,5], 0..5);
        let mut r = Reader::new(&ds);

        let d = r.slice(3, true).unwrap(); // move cursor
        assert_eq!(d, &[1,2,3]);
        assert_eq!(r.cursor, 3);
    }

    #[test]
    fn test_reader_slice_mv_false() {
        let ds = DataSource::create(vec![1,2,3,4,5], 0..5);
        let mut r = Reader::new(&ds);

        let d = r.slice(3, false).unwrap();
        assert_eq!(d, &[1,2,3]);
        assert_eq!(r.cursor, 0); // cursor restored because mv=false
    }

    //
    // -------------------- Reader::set --------------------
    //
    #[test]
    fn test_reader_set() {
        let ds = DataSource::create(vec![1,2,3,4], 100..104);
        let mut r = Reader::new(&ds);

        assert!(r.set(102));
        assert_eq!(r.cursor, 102);

        assert!(!r.set(50)); // outside DataSource range
        assert_eq!(r.cursor, 102);

        assert!(!r.set(200)); // outside Reader range
        assert_eq!(r.cursor, 102);
    }

    //
    // -------------------- Reader::next error --------------------
    //
    #[test]
    fn test_reader_next_error() {
        let ds = DataSource::create(vec![1], 0..1);
        let mut r = Reader::new(&ds);

        r.read8().unwrap(); // now cursor==1

        assert!(r.next().is_err()); // no more bytes
    }

    //
    // -------------------- DataSource::update_slice --------------------
    //
    #[test]
    fn test_datasource_update_slice() {
        let mut ds = DataSource::new(0, 0);
        ds.update_slice(&[1,2,3]);
        assert_eq!(ds.data, vec![1,2,3]);
        assert_eq!(ds.range, 0..3);

        ds.update_slice(&[4,5]);
        assert_eq!(ds.data, vec![1,2,3,4,5]);
        assert_eq!(ds.range, 0..2); // note: your code sets end = start + len (NOT cumulative)
    }

    //
    // -------------------- DataSource::slice boundary tests --------------------
    //
    #[test]
    fn test_datasource_slice_boundary() {
        let ds = DataSource::create(vec![10,20,30,40], 100..104);

        assert!(ds.slice(100..105).is_err());  // end==105 is out of range
        assert!(ds.slice(50..52).is_err());    // start out of range

        assert_eq!(ds.slice(100..104).unwrap(), &[10,20,30,40]);
    }

    //
    // -------------------- Reader::new_sub --------------------
    //
    #[test]
    fn test_reader_new_sub() {
        let ds = DataSource::create(vec![1,2,3,4,5], 10..15);

        let r = Reader::new_sub(&ds, 11..14).unwrap();
        assert_eq!(r.range, 11..14);
        assert_eq!(r.cursor, 11);

        assert!(Reader::new_sub(&ds, 5..8).is_err());   // out of main range
        assert!(Reader::new_sub(&ds, 12..20).is_err()); // end overflow
    }

    //
    // -------------------- Reader::left_range --------------------
    //
    #[test]
    fn test_reader_left_range() {
        let ds = DataSource::create(vec![1,2,3,4], 0..4);
        let mut r = Reader::new(&ds);

        r.forward(2);
        assert_eq!(r.left_range(), 2..4);
    }

    //
    // -------------------- Reader::slice_rest_as_reader --------------------
    //
    #[test]
    fn test_reader_slice_rest_as_reader() {
        let ds = DataSource::create(vec![1,2,3,4], 0..4);
        let mut r = Reader::new(&ds);

        r.forward(1);
        let sub = r.slice_rest_as_reader().unwrap();

        assert_eq!(sub.range, 1..4);
        assert_eq!(sub.preview(3).unwrap(), &[2,3,4]);
    }

    #[test]
    fn test_reader_slice_rest_as_reader_error() {
        let ds = DataSource::create(vec![1], 0..1);
        let mut r = Reader::new(&ds);

        r.forward(1); // cursor at end
        assert!(r.slice_rest_as_reader().is_err());
    }

    //
    // -------------------- Reader::read24 / read128 --------------------
    //
    #[test]
    fn test_reader_read24() {
        let ds = DataSource::create(vec![0x01,0x02,0x03], 0..3);
        let mut r = Reader::new(&ds);

        assert_eq!(r.read24().unwrap(), 0x010203);
    }

    #[test]
    fn test_reader_read128() {
        let bytes: Vec<u8> = (1..=16).collect();
        let ds = DataSource::create(bytes.clone(), 0..16);
        let mut r = Reader::new(&ds);

        let expected = u128::from_be_bytes(bytes.try_into().unwrap());
        assert_eq!(r.read128(true).unwrap(), expected);
    }

    //
    // -------------------- find_crlf edge behavior --------------------
    //
    #[test]
    fn test_find_crlf_edges() {
        assert_eq!(find_crlf(b""), None);
        assert_eq!(find_crlf(b"\r"), None);
        assert_eq!(find_crlf(b"\n"), None);
        assert_eq!(find_crlf(b"\r\r\n"), Some(1));
        assert_eq!(find_crlf(b"xxx\r\n"), Some(3));

        // \r at last byte
        assert_eq!(find_crlf(b"abc\r"), None);
    }

}
