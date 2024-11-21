//! Reference to external tag that will get loaded on access.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Enumerable;
use crate::Result;

#[derive(Default, Debug)]
/// Structure that defines a reference to a tag.
pub struct TagReference {
    /// The index of the data block containing the tag field.
    field_block: i32,
    /// The offset of the tag data block containing the referenced data.
    /// Can be -1 for null references.
    field_offset: u32,
    /// The offset of the tag file name inside the module string table.
    name_offset: u32,
    /// The index of the tag dependency in the tag dependency list.
    /// Can be -1 for null tag references.
    dependency_index: i32,
}

impl Enumerable for TagReference {
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.field_block = reader.read_i32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        self.name_offset = reader.read_u32::<LE>()?;
        self.dependency_index = reader.read_i32::<LE>()?;
        Ok(())
    }
}
