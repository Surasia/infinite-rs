//! Reference to external tag that will get loaded on access.

use std::io::Read;

use byteorder::{ReadBytesExt, LE};

#[derive(Default, Debug)]
/// Structure that defines a reference to a tag.
pub struct TagReference {
    pub field_block: u32,
    pub field_offset: u32,
    pub name_offset: u32,
    pub dependency_index: i32,
}

impl TagReference {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        self.name_offset = reader.read_u32::<LE>()?;
        self.dependency_index = reader.read_i32::<LE>()?;
        Ok(())
    }
}
