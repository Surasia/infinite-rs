//! Reference to binary blob inside tag that isn't defined by a structure.

use std::io::BufRead;

use byteorder::{ReadBytesExt, LE};

#[derive(Default, Debug)]
/// Structure that defines a reference to a blob of data inside tag data.
pub struct TagDataReference {
    /// The index of the tag struct containing the tag field.
    pub parent_struct_index: i32,
    /// Unknown: seems to vary.
    pub unknown: i32,
    /// The index of the tag struct containing the referenced data.
    /// Can be -1 for null references.
    pub target_index: i32,
    /// The index of the data block containing the tag field.
    pub field_block: u32,
    /// The offset of the tag field inside the data block.
    pub field_offset: u32,
}

impl TagDataReference {
    /// Allocate new TagDataReference and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the tag data reference from the given readers implementing "Read".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `Read` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Err` if an I/O error occurs
    /// or if the header data is invalid.
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.parent_struct_index = reader.read_i32::<LE>()?;
        self.unknown = reader.read_i32::<LE>()?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
