use crate::common::Context;

pub trait ProtocolParser {
    fn parse(ctx: &mut Context, reader: &mut crate::common::io::Reader);
}