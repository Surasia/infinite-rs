//! Common errors used throughout `infinite-rs`.
use std::num::TryFromIntError;

use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur when reading a module file.
pub enum ModuleError {
    /// Incorrect magic found in the module file, when not "ucsh".
    #[error("Incorrect magic found! Expected '0x64686F6D', found {0:#X}!")]
    IncorrectMagic(u32),
    /// Incorrect version found in the module file, when not 53.
    /// Module version 53 is the only fully supported module version, however others should also work.
    #[error("Incorrect version found! Expected '53', found {0}!")]
    IncorrectVersion(i32),
    /// This is not supposed to happen, however if it does, it means that the module file is corrupted.
    /// The error exists to act as an assert.
    #[error("Module file block index must be non-negative, found {0}")]
    NegativeBlockIndex(i32),
}

#[derive(Error, Debug)]
/// Errors that can occur when reading a tag file.
pub enum TagError {
    /// Incorrect magic found in the tag file, when not "mohd".
    #[error("Incorrect magic found! Expected '0x68736375', found {0:#X}!")]
    IncorrectMagic(u32),
    /// Incorrect version found in the module file, when not 27.
    /// Tag version 27 seems to be consistent across all versions of Infinite, also being the same as Halo 5, although they have seperate structures.
    #[error("Incorrect version found! Expected '27', found {0}!")]
    IncorrectVersion(i32),
}

#[derive(Error, Debug)]
/// Errors that can occur when decompressing data.
pub enum DecompressionError {
    /// Occurs when the buffer size is too small to hold the decompressed data, which should never happen at least in Infinite's modules.
    #[error("Buffer size overflow")]
    BufferSizeOverflow,
    /// Any error code other than 0 indicates that the decompression failed, with the error code being from the Kraken decompressor.
    #[error("Decompression failed with error code {0}")]
    DecompressionFailed(i32),
}

#[derive(Error, Debug)]
/// Standard error type used throughout `infinite-rs`.
pub enum Error {
    /// Any error originating from `std::io`, such as `UnexpectedEOF`.
    #[error("Failed to read from buffer!")]
    ReadError(#[from] std::io::Error),
    /// Error that can occur in `read_fixed_string` if invalid UTF-8 encoding is found.
    #[error("Incorrect UTF-8 encoding found when reading string!")]
    Utf8ReadingError(#[from] std::string::FromUtf8Error),
    /// Errors that can occur while decompressing using Kraken.
    #[error("Error occured while decompressing!")]
    DecompressionError(#[from] DecompressionError),
    /// Errors that can occur while loading a module file.
    #[error("Error occured while loading a module!")]
    ModuleError(#[from] ModuleError),
    /// The error type returned when a checked integral type conversion fails.
    #[error("Integer conversion failed!")]
    TryFromIntError(#[from] TryFromIntError),
    /// Errors that can occur while loading a tag file.
    #[error("Error occured while loading a tag!")]
    TagError(#[from] TagError),
}
