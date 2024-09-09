//! Hierarchical structure entry of tag.

use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use num_enum::TryFromPrimitive;
use std::io::BufRead;

use crate::common::extensions::Readable;

#[derive(Default, Debug, TryFromPrimitive)]
#[repr(u16)]
/// Enum defining what the tag struct is pointing to.
pub enum TagStructType {
    #[default]
    /// "Root" of structure.
    MainStruct,
    /// An array of items in structure.
    TagBlock,
    /// Reference to child resource.
    Resource,
    /// Reference to "external" resource.
    Custom,
    /// Unknown
    Literal,
}

#[derive(Default, Debug)]
/// Structure defining the hierarchical order of info in tags.
pub struct TagStruct {
    /// GUID of the structure referenced.
    pub guid: u128,
    /// Where the structure is located.
    pub struct_type: TagStructType,
    /// Unknown (but important)
    pub unknown: u16,
    /// For main struct and tag block structs, the index of the block containing the struct.
    /// For resource structs, index of the resource.
    /// Can be -1 if the tag field doesn't point to anything.
    pub target_index: i32,
    /// The index of the data block containing the tag field which refers to this struct.
    /// Can be -1 for the main struct.
    pub field_block: u32,
    /// The offset of the tag field inside the data block.
    pub field_offset: u32,
}

impl Readable for TagStruct {
    /// Reads the tag structure from a given buffer reader.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<&[u8]>` that implements `BufRead` from which to read the data
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `std::io::Error` if any read fails.
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.guid = reader.read_u128::<LE>()?;
        self.struct_type = TagStructType::try_from(reader.read_u16::<LE>()?).unwrap();
        self.unknown = reader.read_u16::<LE>()?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
