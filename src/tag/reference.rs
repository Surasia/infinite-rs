//! Reference to external tag that will get loaded on access.

use std::io::BufRead;

use byteorder::{ReadBytesExt, LE};

#[derive(Default, Debug)]
/// Structure that defines a reference to a tag.
pub struct TagReference {
    /// The index of the data block containing the tag field.
    pub field_block: u32,
    /// The offset of the tag data block containing the referenced data.
    /// Can be -1 for null references.
    pub field_offset: u32,
    /// The offset of the tag file name inside the module string table.
    pub name_offset: u32,
    /// The index of the tag dependency in the tag dependency list.
    /// Can be -1 for null tag references.
    pub dependency_index: i32,
}

impl TagReference {
    /// Allocate new TagReference and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the tag reference from the given readers implementing "Read".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `Read` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Err` if an I/O error occurs
    /// or if the header data is invalid.
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        self.name_offset = reader.read_u32::<LE>()?;
        self.dependency_index = reader.read_i32::<LE>()?;
        Ok(())
    }
}
