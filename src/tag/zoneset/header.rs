//! Zoneset header containing info on the entirety of the zoneset structure.

use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
/// Zoneset header used to read tags.
pub struct TagZonesetHeader {
    pub version: i32,
    pub zoneset_count: u32,
    pub footer_count: u32,
    pub parents: u32,
}

impl TagZonesetHeader {
    /// Reads the tag zoneset header from the given readers implementing "Read".
    ///
    /// This function populates the fields of the `TagZonesetHeader` struct by reading
    /// various data types from the provided `reader`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `Read` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Err` if an I/O error occurs
    /// or if the header data is invalid.
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.version = reader.read_i32::<LE>()?;
        self.zoneset_count = reader.read_u32::<LE>()?;
        self.footer_count = reader.read_u32::<LE>()?;
        self.parents = reader.read_u32::<LE>()?;
        Ok(())
    }
}
