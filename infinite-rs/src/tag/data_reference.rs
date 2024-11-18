//! Reference to binary blob inside tag that isn't defined by a structure.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Readable;
use crate::Result;

#[derive(Default, Debug)]
/// Structure that defines a reference to a blob of data inside tag data.
pub struct TagDataReference {
    /// The index of the tag struct containing the tag field.
    pub parent_struct_index: i32,
    /// Unknown: seems to vary (maybe enum?).
    unknown: i32,
    /// The index of the tag struct containing the referenced data.
    /// Can be -1 for null references.
    pub target_index: i32,
    /// The index of the data block containing the tag field.
    pub field_block: i32,
    /// The offset of the tag field inside the data block.
    pub field_offset: u32,
}

impl Readable for TagDataReference {
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead,
    {
        self.parent_struct_index = reader.read_i32::<LE>()?;
        self.unknown = reader.read_i32::<LE>()?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_i32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
