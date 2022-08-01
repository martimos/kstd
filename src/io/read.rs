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

/// Reads exactly [`$count`] bytes from the [`$source`] and stores them in
/// an array of size [`$count`]. This macro evaluates to that array.
///
/// ```rust
/// use kstd::io::cursor::Cursor;
/// use kstd::read_bytes;
/// use kstd::io::Read;
/// fn foo() -> kstd::io::Result<()> {
///     let data = vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///     let mut c = Cursor::new(data);
///     let buf = read_bytes!(c, 6);
///     assert_eq!([0, 1, 2, 3, 4, 5], buf);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! read_bytes {
    ($source:expr, $count:expr) => {{
        let mut buf = [0_u8; $count];
        $source.read_exact(&mut buf)?;
        buf
    }};
}

/// Reads a null-terminated ASCII string from the given source.
/// If the string is not null-terminated within [`$count`] bytes,
/// this will evaluate to a string with all [`$count`] bytes.
/// The string is created with [`String::from_utf8_lossy`].
#[macro_export]
macro_rules! read_null_terminated_string {
    ($source:expr, $count:expr) => {{
        let data = read_bytes!($source, $count);
        let pos = data.iter().position(|&b| b == 0).unwrap_or(data.len());
        alloc::string::ToString::to_string(&String::from_utf8_lossy(&data[0..pos]))
    }};
}

/// Reads a single byte from the given source.
#[macro_export]
macro_rules! read_u8 {
    ($source:expr) => {{
        u8::from_be_bytes(read_bytes!($source, 1))
    }};
}

/// Reads a single 16-bit integer from the given source. Uses big endian.
#[macro_export]
macro_rules! read_be_u16 {
    ($source:expr) => {{
        u16::from_be_bytes(read_bytes!($source, 2))
    }};
}

/// Reads a single 32-bit integer from the given source. Uses big endian.
#[macro_export]
macro_rules! read_be_u32 {
    ($source:expr) => {{
        u32::from_be_bytes(read_bytes!($source, 4))
    }};
}

/// Reads a single 64-bit integer from the given source. Uses big endian.
#[macro_export]
macro_rules! read_be_u64 {
    ($source:expr) => {{
        u64::from_be_bytes(read_bytes!($source, 8))
    }};
}

/// Reads a single 16-bit integer from the given source. Uses little endian.
#[macro_export]
macro_rules! read_le_u16 {
    ($source:expr) => {{
        u16::from_le_bytes(read_bytes!($source, 2))
    }};
}

/// Reads a single 32-bit integer from the given source. Uses little endian.
#[macro_export]
macro_rules! read_le_u32 {
    ($source:expr) => {{
        u32::from_le_bytes(read_bytes!($source, 4))
    }};
}

/// Reads a single 64-bit integer from the given source. Uses little endian.
#[macro_export]
macro_rules! read_le_u64 {
    ($source:expr) => {{
        u64::from_le_bytes(read_bytes!($source, 8))
    }};
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::io::cursor::Cursor;
    use crate::io::read::Read;
    use crate::io::testing::SingleRead;

    #[test]
    fn test_read_exact() {
        let data = vec![0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let r = SingleRead::new(data);
        let mut c = Cursor::new(r);
        let mut buf = vec![0_u8; 5];
        c.read_exact(&mut buf).unwrap();
        assert_eq!(vec![0_u8, 1, 2, 3, 4], buf);
    }
}
