use crate::io::{Error, Result};

pub trait Write<T> {
    fn write(&mut self, buf: &dyn AsRef<[T]>) -> Result<usize>;

    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &dyn AsRef<[T]>) -> Result<()> {
        let mut buffer = buf.as_ref();

        while !buffer.is_empty() {
            match self.write(&buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buffer;
                    buffer = &tmp[n..];
                }
                Err(e) => return Err(e),
            }
        }
        if buffer.is_empty() {
            Ok(())
        } else {
            Err(Error::WriteError)
        }
    }
}

pub trait WriteAt<T> {
    fn write_at(&mut self, offset: u64, buf: &dyn AsRef<[T]>) -> Result<usize>;
}
