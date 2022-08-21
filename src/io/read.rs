use crate::io::{Error, Result};

pub trait Read<T> {
    /// Reads from this source once and places the result in [`buf`].
    /// Returns the number of bytes read. This method does not guarantee
    /// to read [`buf`] fully. For that, see [`Read::read_exact`].
    fn read(&mut self, buf: &mut dyn AsMut<[T]>) -> Result<usize>;

    /// Reads the full buffer from this source. Might block if the
    /// implementation of [`Read::read`] blocks. If this returns
    /// [`Result::Ok`], then the full buffer has been read. If it
    /// returns [`Result::Err`], then either an error occurred during
    /// [`Read::read`] or the source is at EOF, in which case
    /// [`Error::PrematureEndOfInput`] is returned.
    fn read_exact(&mut self, buf: &mut dyn AsMut<[T]>) -> Result<()> {
        let mut buffer = buf.as_mut();

        while !buffer.is_empty() {
            match self.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buffer;
                    buffer = &mut tmp[n..];
                }
                Err(e) => return Err(e),
            }
        }
        if buffer.is_empty() {
            Ok(())
        } else {
            Err(Error::PrematureEndOfInput)
        }
    }
}

pub trait ReadAt<T> {
    /// Reads from this source at the specified offset and places the result in [`buf`].
    /// This method does not guarantee to read [`buf`] fully. If that is your requirement,
    /// create a [`Cursor`] and use [`Read::read_exact`].
    fn read_at(&self, offset: u64, buf: &mut dyn AsMut<[T]>) -> Result<usize>;
}
