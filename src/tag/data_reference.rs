//! Reference to binary blob inside tag that isn't defined by a structure.

use std::io::Read;

use byteorder::{ReadBytesExt, LE};

#[derive(Default, Debug)]
/// Structure that defines a reference to a blob of data inside tag data.
pub struct TagDataReference {
    pub parent_struct_index: i32,
    pub unknown: i32,
    pub target_index: i32,
    pub field_block: u32,
    pub field_offset: u32,
}

impl TagDataReference {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.parent_struct_index = reader.read_i32::<LE>()?;
        self.unknown = reader.read_i32::<LE>()?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
