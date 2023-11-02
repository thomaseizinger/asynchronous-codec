use std::io;
use bytes::{Buf, Bytes, BytesMut};
use crate::{Decoder, Encoder};

/// Prefixes each byte slice with its length as an [unsigned, variable-length integer](https://github.com/multiformats/unsigned-varint).
#[derive(Default)]
pub struct UviCodec;

impl Encoder for UviCodec {
    type Item<'a> = &'a [u8];
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = item.len();

        let mut buffer = unsigned_varint::encode::usize_buffer();
        let encoded = unsigned_varint::encode::usize(len, &mut buffer);

        dst.extend_from_slice(encoded);
        dst.extend_from_slice(item);

        Ok(())
    }
}

impl Decoder for UviCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let (len, remaining) = match unsigned_varint::decode::usize(src) {
            Ok((len, remaining)) => (len, remaining),
            Err(unsigned_varint::decode::Error::Insufficient) => return Ok(None),
            Err(e ) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        };
        let consumed = src.len() - remaining.len();
        src.advance(consumed);

        Ok(Some(src.split_to(len).freeze()))
    }
}
