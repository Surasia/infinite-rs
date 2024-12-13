//! Reference to external tag that will get loaded on access.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Enumerable;
use crate::Result;

#[derive(Default, Debug)]
/// Structure that defines a reference to a tag.
pub struct TagReference {
    /// The index of the data block containing the tag field.
    pub field_block: i32,
    /// The offset of the tag data block containing the referenced data.
    /// Can be -1 for null references.
    pub field_offset: u32,
    /// The offset of the tag file name inside the tag string table.
    pub(crate) name_offset: u32,
    /// The index of the tag dependency in the tag dependency list.
    /// Can be -1 for null tag references.
    pub dependency_index: i32,
    /// Tag name of the reference, located at the position of the [`Self::name_offset`] in the tag string table.
    /// This only contains values before Season 3.
    pub name: Option<String>,
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
