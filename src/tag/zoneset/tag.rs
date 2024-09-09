//! Zoneset tag containing its name and reference ID.

use crate::common::extensions::Readable;
use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

#[derive(Default, Debug)]
pub struct TagZonesetTag {
    /// Unknown, id of the tag?
    pub global_id: i32,
    /// Name of the zoneset.
    pub string_id: i32,
}

impl Readable for TagZonesetTag {
    /// Allocate new TagZonesetTag and set it to default values.
    /// Reads the tag zoneset tag from the given readers implementing "BufRead".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.global_id = reader.read_i32::<LE>()?;
        self.string_id = reader.read_i32::<LE>()?;
        Ok(())
    }
}
