use bytes::{BufMut, BytesMut};

use tokio_util::codec::{Decoder, Encoder, LinesCodec, LinesCodecError};

pub struct TelnetCodec {
    // We just wrap the LinesCodec - the decoder works out of the box for us, in some cases the
    // encoder does not because want to send without a terminating newline
    inner: LinesCodec,
}

impl TelnetCodec {
    pub fn new() -> Self {
        Self {
            inner: LinesCodec::new(),
        }
    }
}

impl Decoder for TelnetCodec {
    type Item = String;
    type Error = LinesCodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.inner.decode(src)
    }
}

impl<T> Encoder<T> for TelnetCodec
where
    T: AsRef<str>,
{
    type Error = LinesCodecError;

    fn encode(&mut self, item: T, buf: &mut BytesMut) -> Result<(), Self::Error> {
        self.inner.encode(item, buf)
    }
}

pub struct Prompt<T>
where
    T: AsRef<str>,
{
    inner: T,
}

impl<T> Prompt<T>
where
    T: AsRef<str>,
{
    pub fn new(line: T) -> Self {
        Self { inner: line }
    }
}

impl<T> Encoder<Prompt<T>> for TelnetCodec
where
    T: AsRef<str>,
{
    type Error = LinesCodecError;

    fn encode(&mut self, item: Prompt<T>, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let line: &str = item.inner.as_ref();
        // OLD: buf.reserve(line.len() + 1);
        buf.reserve(line.len());
        buf.put(line.as_bytes());
        // OLD: buf.put_u8(b'\n');
        Ok(())
    }
}
