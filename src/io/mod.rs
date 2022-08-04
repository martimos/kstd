use alloc::vec::Vec;
use derive_more::Display;

pub use read::*;

pub mod block;
pub mod cursor;
pub mod macros;
pub mod read;
pub mod seek;
pub mod testing;
pub mod write;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// The offset is out of bounds or does not meet
    /// other restrictions.
    InvalidOffset,
    /// The provided buffer was too small to fit all the data
    /// it needs to fit.
    BufferTooSmall,
    /// The input ended although it was expected to
    /// produce more data.
    PrematureEndOfInput,
    /// The requested block is not present on the device.
    NoSuchBlock,
    /// The requested function is not implemented for this
    /// I/O component.
    NotImplemented,
    /// The requested entity is not present on the device or
    /// the registry does not hold an entry matching the
    /// criteria.
    NotFound,
    /// An entry or entity was found, but there must not be
    /// one in order for the operation to continue or succeed.
    ExistsButShouldNot,
    /// The provided address is invalid.
    BadAddress,
    /// An invalid value was encountered while decoding.
    DecodeError,
    /// The magic value in the data does not match the expected one.
    InvalidMagicNumber,
    /// The data provided was not coherent or a checksum did not
    /// match the data.
    IncoherentData,
    /// The provided argument was invalid.
    InvalidArgument,
    /// The found entry is a file.
    IsFile,
    /// The found entry is a directory.
    IsDir,
}

pub trait ReadAt<T> {
    /// Reads from this source at the specified offset and places the result in [`buf`].
    /// This method does not guarantee to read [`buf`] fully. If that is your requirement,
    /// create a [`Cursor`] and use [`Read::read_exact`].
    fn read_at(&self, offset: u64, buf: &mut dyn AsMut<[T]>) -> Result<usize>;
}

impl<T> ReadAt<T> for &Vec<T>
where
    T: Copy,
{
    fn read_at(&self, offset: u64, buf: &mut dyn AsMut<[T]>) -> Result<usize> {
        let buffer = buf.as_mut();

        let start = offset as usize;
        let end = start + buffer.len();
        if end > self.len() {
            // if `end` is within bounds, start is within bounds, too
            return Err(Error::InvalidOffset);
        }

        buffer.copy_from_slice(&self[start..end]);
        Ok(buffer.len())
    }
}

impl<T> ReadAt<T> for Vec<T>
where
    T: Copy,
{
    fn read_at(&self, offset: u64, buf: &mut dyn AsMut<[T]>) -> Result<usize> {
        ReadAt::<T>::read_at(&self, offset, buf)
    }
}

pub trait WriteAt<T> {
    fn write_at(&mut self, offset: u64, buf: &dyn AsRef<[T]>) -> Result<usize>;
}
