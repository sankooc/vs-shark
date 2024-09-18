use crate::common::io::Reader;

trait Decoder {
    fn decode(reader: &Reader);
}