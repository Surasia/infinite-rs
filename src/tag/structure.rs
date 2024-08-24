//! Hierarchical structure of tag.

use byteorder::{ReadBytesExt, LE};
use num_enum::TryFromPrimitive;
use std::io::Read;

#[derive(Default, Debug, TryFromPrimitive)]
#[repr(u16)]
/// Enum defining what the tag struct is pointing to.
pub enum TagStructType {
    #[default]
    MainStruct,
    TagBlock,
    Resource,
    Custom,
    Literal,
}

#[derive(Default, Debug)]
/// Structure defining the hierarchical order of info in tags.
pub struct TagStruct {
    pub guid: u128,
    pub struct_type: TagStructType,
    pub unknown: u16,
    pub target_index: i32,
    pub field_block: u32,
    pub field_offset: u32,
}

impl TagStruct {
    /// Reads the tag structure from a given buffer reader.
    ///
    /// This function populates the `TagStruct` fields by reading data from the provided buffer reader.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<&[u8]>` from which to read the data
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `std::io::Error` if any read fails.
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.guid = reader.read_u128::<LE>()?;
        self.struct_type = TagStructType::try_from(reader.read_u16::<LE>()?).unwrap();
        self.unknown = reader.read_u16::<LE>()?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
