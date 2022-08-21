// more or less copied from the Rust stdlib

use crate::io::read::Read;
use crate::io::{Error, Seek, Write};
use crate::io::{Result, SeekFrom};

pub struct Cursor<T> {
    inner: T,
    pos: u64,
}

impl<T> Cursor<T> {
    pub const fn new(inner: T) -> Self {
        Self { inner, pos: 0 }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub const fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn position(&self) -> u64 {
        self.pos
    }

    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
}

impl<T> Cursor<T>
where
    T: AsRef<[u8]>,
{
    pub fn remaining_slice(&self) -> &[u8] {
        let len = self.pos.min(self.inner.as_ref().len() as u64);
        &self.inner.as_ref()[(len as usize)..]
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.inner.as_ref().len() as u64
    }
}

impl<T> Clone for Cursor<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            pos: self.pos,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
        self.pos = source.pos;
    }
}

impl<T> Seek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(n) => n,
            SeekFrom::End(n) => {
                let p = (self.inner.as_ref().len() as i64) - n;
                if p < 0 {
                    return Err(Error::InvalidOffset);
                }
                p as u64
            }
            SeekFrom::Current(n) => self.pos as u64 + n as u64,
        };
        if new_pos >= self.inner.as_ref().len() as u64 {
            Err(Error::InvalidOffset)
        } else {
            self.pos = new_pos;
            Ok(self.pos)
        }
    }
}

impl<T> Read<u8> for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut dyn AsMut<[u8]>) -> Result<usize> {
        let data = self.inner.as_ref();
        let buffer = buf.as_mut();
        let len = (data.len() as u64 - self.pos).min(buffer.len() as u64) as usize;
        let target_buffer = &mut buffer[..len];
        target_buffer.copy_from_slice(&data[(self.pos as usize)..(self.pos as usize + len)]);
        self.pos += len as u64;
        Ok(len)
    }
}

impl<T> Write<u8> for Cursor<T>
where
    T: AsMut<[u8]>,
{
    fn write(&mut self, buf: &dyn AsRef<[u8]>) -> Result<usize> {
        let data = self.inner.as_mut();
        let buffer = buf.as_ref();
        let len = (data.len() as u64 - self.pos).min(buffer.len() as u64) as usize;
        let target_buffer = &mut data[(self.pos as usize)..(self.pos as usize + len)];
        target_buffer.copy_from_slice(&buffer[..len]);
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_read() {
        let data = &[0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut buf = [0u8; 4];
        let mut c = Cursor::new(data);

        let read1 = c.read(&mut buf);
        assert_eq!(Ok(4), read1);
        assert_eq!(&[0, 1, 2, 3], &buf);

        let read2 = c.read(&mut buf);
        assert_eq!(Ok(4), read2);
        assert_eq!(&[4, 5, 6, 7], &buf);

        let read3 = c.read(&mut buf);
        assert_eq!(Ok(2), read3);
        assert_eq!(&[8, 9, 6, 7], &buf);
    }

    #[test]
    fn test_read_vec() {
        let data = vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut buf = [0u8; 4];
        let mut c = Cursor::new(data);

        let read1 = c.read(&mut buf);
        assert_eq!(Ok(4), read1);
        assert_eq!(&[0, 1, 2, 3], &buf);

        let read2 = c.read(&mut buf);
        assert_eq!(Ok(4), read2);
        assert_eq!(&[4, 5, 6, 7], &buf);

        let read3 = c.read(&mut buf);
        assert_eq!(Ok(2), read3);
        assert_eq!(&[8, 9, 6, 7], &buf);
    }
}
