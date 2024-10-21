//! Hierarchical structure entry of tag.

use byteorder::{ReadBytesExt, LE};
use num_enum::TryFromPrimitive;
use std::io::BufRead;

use crate::common::{errors::Error, extensions::Readable};

#[derive(Default, Debug, TryFromPrimitive)]
#[repr(u16)]
/// Enum defining what the tag struct is pointing to.
pub(super) enum TagStructType {
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
pub(super) struct TagStruct {
    /// GUID of the structure referenced.
    pub(super) guid: u128,
    /// Where the structure is located.
    pub(super) struct_type: TagStructType,
    /// Unknown (but important)
    unknown: u16,
    /// For main struct and tag block structs, the index of the block containing the struct.
    /// For resource structs, index of the resource.
    /// Can be -1 if the tag field doesn't point to anything.
    pub(super) target_index: i32,
    /// The index of the data block containing the tag field which refers to this struct.
    /// Can be -1 for the main struct.
    pub(super) field_block: u32,
    /// The offset of the tag field inside the data block.
    pub(super) field_offset: u32,
}

impl Readable for TagStruct {
    fn read<R>(&mut self, reader: &mut R) -> Result<(), Error>
    where
        R: BufRead,
    {
        self.guid = reader.read_u128::<LE>()?;
        self.struct_type = TagStructType::try_from(reader.read_u16::<LE>()?).unwrap();
        self.unknown = reader.read_u16::<LE>()?;
        self.target_index = reader.read_i32::<LE>()?;
        self.field_block = reader.read_u32::<LE>()?;
        self.field_offset = reader.read_u32::<LE>()?;
        Ok(())
    }
}
