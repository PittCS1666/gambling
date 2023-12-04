use std::io::{Read, Write};
use anyhow::anyhow;
/// A middle layer that splits a stream into messages.
#[derive(Debug)]
pub struct MessageProto<T> {
    inner: T,
    size_limit: usize,
}
impl<T> MessageProto<T> {
    /// Sets received datagram size limitation.
    pub fn set_size_limit(&mut self, new: usize) -> usize {
        std::mem::replace(&mut self.size_limit, new)
    }

    /// Creates a new [`MessageProto`] with provided stream.
    pub fn new(inner: T, size_limit: usize) -> Self {
        Self { inner, size_limit }
    }
}
impl<T: Read + Unpin> MessageProto<T> {
    /// Receives a datagram from the stream.
    pub fn recv(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut len = [0u8; 8];
        self.inner.read_exact(&mut len)?;
        let len = u64::from_le_bytes(len) as usize;
        if len > self.size_limit {
            return Err(anyhow!("datagram is too big ({} bytes)", len));
        }
        let mut blob = vec![0u8; len];
        self.inner.read_exact(&mut blob)?;

        Ok(blob)
    }
}
impl<T: Write + Unpin> MessageProto<T> {
    /// Sends a datagram to the stream.
    pub fn send(&mut self, blob: &[u8]) -> anyhow::Result<()> {
        let len = u64::to_le_bytes(blob.len() as u64);
        self.inner.write_all(&len)?;
        self.inner.write_all(blob)?;

        Ok(())
    }
}
impl<T> AsRef<T> for MessageProto<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
impl<T> AsMut<T> for MessageProto<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
impl<T: Read + Write + Unpin> From<T> for MessageProto<T> {
    fn from(inner: T) -> Self {
        Self::new(inner, 6 * 1024 * 1024)
    }
}