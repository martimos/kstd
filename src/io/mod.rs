use derive_more::Display;

pub use read::*;

pub mod block;
pub mod cursor;
pub mod macros;
pub mod read;
pub mod seek;
pub mod testing;
pub mod write;

pub use crate::io::read::*;
pub use crate::io::seek::*;
pub use crate::io::write::*;

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
    /// The found entry is a file, but shouldn't be.
    IsFile,
    /// The found entry is a directory, but shouldn't be.
    IsDir,
    /// The found entry is a symbolic link, but shouldn't be.
    IsSymLink,
    /// An unexpected error occurred during the write, or the write
    /// couldn't be completed.
    WriteError,
}
