//! Zoneset header containing info on the entirety of the zoneset structure.

use crate::common::extensions::{BufReaderExt, Readable};
use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

#[derive(Default, Debug)]
/// Zoneset header used to read tags.
pub struct TagZonesetHeader {
    /// Name of the zoneset that this tag belongs to.
    pub string_id: i32,
    /// Number of tags to load for this zoneset.
    pub zoneset_count: u32,
    /// Unknown: 4 byte items?
    pub footer_count: u32,
    /// Number of parent tags to load for this zoneset.
    pub parents: u32,
}

impl Readable for TagZonesetHeader {
    /// Reads the tag zoneset header from the given readers implementing "BufRead".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read<R: BufRead + BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.string_id = reader.read_i32::<LE>()?;
        self.zoneset_count = reader.read_u32::<LE>()?;
        self.footer_count = reader.read_u32::<LE>()?;
        self.parents = reader.read_u32::<LE>()?;
        Ok(())
    }
}
