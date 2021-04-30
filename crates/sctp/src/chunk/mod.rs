mod chunk_abort;
mod chunk_header;
mod chunk_type;

use crate::error::Error;

use bytes::{Bytes, BytesMut};
use std::marker::Sized;

pub(crate) trait Chunk {
    fn unmarshal(raw: &Bytes) -> Result<Self, Error>
    where
        Self: Sized;
    fn marshal_to(&self, buf: &mut BytesMut) -> Result<usize, Error>;
    fn check(&self) -> Result<bool, Error>;
    fn value_length(&self) -> usize;

    fn marshal(&self) -> Result<Bytes, Error> {
        let capacity = 4 + self.value_length();
        let mut buf = BytesMut::with_capacity(capacity);
        self.marshal_to(&mut buf)?;
        Ok(buf.freeze())
    }
}
