//! Module block containing info relating to Kraken compression.

use byteorder::{ReadBytesExt, LE};
use std::{fs::File, io::BufReader};

#[derive(Default, Debug)]
/// Module block containing info relating to Kraken compression.
/// Is used to determine how to read bytes in module_file.
/// Caution: Max size is 2GBs, Kraken limitation!
pub struct ModuleBlockEntry {
    pub compressed_offset: i32,
    pub compressed_size: i32,
    pub decompressed_offset: i32,
    pub decompressed_size: i32,
    pub is_compressed: bool,
}

impl ModuleBlockEntry {
    /// Reads the module block entry data from the provided buffered reader.
    ///
    /// This method populates the fields of the `ModuleBlockEntry` struct by reading
    /// values from the given `BufReader<File>`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        self.compressed_offset = reader.read_i32::<LE>()?;
        self.compressed_size = reader.read_i32::<LE>()?;
        self.decompressed_offset = reader.read_i32::<LE>()?;
        self.decompressed_size = reader.read_i32::<LE>()?;
        self.is_compressed = reader.read_u32::<LE>()? != 0;
        Ok(())
    }
}
