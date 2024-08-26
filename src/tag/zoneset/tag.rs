//! Zoneset tag containing its name and reference ID.

use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
pub struct TagZonesetTag {
    /// Unknown, id of the tag?
    pub global_id: i32,
    /// Name of the zoneset.
    pub string_id: i32,
}

impl TagZonesetTag {
    /// Allocate new TagZonesetTag and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the tag zoneset tag from the given readers implementing "Read".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `Read` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.global_id = reader.read_i32::<LE>()?;
        self.string_id = reader.read_i32::<LE>()?;
        Ok(())
    }
}
