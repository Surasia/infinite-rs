//! Module block containing info relating to Kraken compression.

use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Readable;

#[derive(Default, Debug)]
/// Module block containing info relating to Kraken compression.
/// Is used to determine how to read bytes in module_file.
///
/// Caution: Max size is 2GBs, engine limitation.
pub struct ModuleBlockEntry {
    /// Offset of compressed data inside module (after file_data_offset in module).
    pub compressed_offset: i32,
    /// Size in bytes of compressed data inside module.
    pub compressed_size: i32,
    /// Offset of decompressed data inside decompression buffer.
    pub decompressed_offset: i32,
    /// Size in bytes of decompression buffer.
    pub decompressed_size: i32,
    /// Boolean indicating if block is compressed or not.
    /// Tags can be made up of both compressed and decompressed blocks.
    pub is_compressed: bool,
}

impl Readable for ModuleBlockEntry {
    /// Reads the module block entry data from the provided buffered reader.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.compressed_offset = reader.read_i32::<LE>()?;
        self.compressed_size = reader.read_i32::<LE>()?;
        self.decompressed_offset = reader.read_i32::<LE>()?;
        self.decompressed_size = reader.read_i32::<LE>()?;
        self.is_compressed = reader.read_u32::<LE>()? != 0;
        Ok(())
    }
}
