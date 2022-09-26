use derive_more::Display;

pub use read::*;

pub mod block;
pub mod cursor;
pub mod macros;
pub mod read;
pub mod seek;
pub mod write;

pub use crate::io::read::*;
pub use crate::io::seek::*;
pub use crate::io::write::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// The offset is out of bounds or does not meet
    /// other restrictions.
    #[display(fmt = "invalid offset")]
    InvalidOffset,
    /// The provided buffer was too small to fit all the data
    /// it needs to fit.
    #[display(fmt = "buffer too small")]
    BufferTooSmall,
    /// The input ended although it was expected to
    /// produce more data.
    #[display(fmt = "premature end of input")]
    PrematureEndOfInput,
    /// The requested block is not present on the device.
    #[display(fmt = "no such block")]
    NoSuchBlock,
    /// The requested function is not implemented for this
    /// I/O component.
    #[display(fmt = "not implemented")]
    NotImplemented,
    /// The requested entity is not present on the device or
    /// the registry does not hold an entry matching the
    /// criteria.
    #[display(fmt = "not found")]
    NotFound,
    /// An entry or entity was found, but there must not be
    /// one in order for the operation to continue or succeed.
    #[display(fmt = "exists, but it should not")]
    ExistsButShouldNot,
    /// The provided address is invalid.
    #[display(fmt = "bad address")]
    BadAddress,
    /// An invalid value was encountered while decoding.
    #[display(fmt = "decode error")]
    DecodeError,
    /// The magic value in the data does not match the expected one.
    #[display(fmt = "invalid magic number")]
    InvalidMagicNumber,
    /// The data provided was not coherent or a checksum did not
    /// match the data.
    #[display(fmt = "data is incoherent")]
    IncoherentData,
    /// The provided argument was invalid.
    #[display(fmt = "invalid argument")]
    InvalidArgument,
    /// The found entry is a file, but shouldn't be.
    #[display(fmt = "is a file")]
    IsFile,
    /// The found entry is a directory, but shouldn't be.
    #[display(fmt = "is a directory")]
    IsDir,
    /// The found entry is a symbolic link, but shouldn't be.
    #[display(fmt = "is a symlink")]
    IsSymLink,
    /// An unexpected error occurred during the write, or the write
    /// couldn't be completed.
    #[display(fmt = "write error")]
    WriteError,
}

impl core::error::Error for Error {}
