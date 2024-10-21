//! Module block containing info relating to Kraken compression.
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::{errors::Error, extensions::Readable};

#[derive(Default, Debug)]
/// Represents a module block entry containing information related to Kraken compression.
/// This struct is used to determine how to read bytes in `module_file`.
pub(super) struct ModuleBlockEntry {
    /// Offset in bytes of compressed data inside the module (after `file_data_offset` in the module).
    pub(super) compressed_offset: u32,
    /// Size in bytes of compressed data inside the module.
    pub(super) compressed_size: u32,
    /// Offset in bytes of decompressed data inside the decompression buffer.
    pub(super) decompressed_offset: u32,
    /// Size in bytes of the decompression buffer.
    pub(super) decompressed_size: u32,
    /// Boolean indicating if the block is compressed or not.
    /// Tags can be made up of both compressed and decompressed blocks.
    pub(super) is_compressed: bool,
}

impl Readable for ModuleBlockEntry {
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<(), Error>
    where
        R: BufRead,
    {
        self.compressed_offset = reader.read_u32::<LE>()?;
        self.compressed_size = reader.read_u32::<LE>()?;
        self.decompressed_offset = reader.read_u32::<LE>()?;
        self.decompressed_size = reader.read_u32::<LE>()?;
        self.is_compressed = reader.read_u32::<LE>()? != 0;
        Ok(())
    }
}
