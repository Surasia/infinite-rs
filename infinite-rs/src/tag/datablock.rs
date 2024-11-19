//! Tag datablock specifying the section for tag structs.

use byteorder::{ReadBytesExt, LE};
use num_enum::TryFromPrimitive;
use std::io::BufRead;

use crate::common::errors::{Error, TagError};
use crate::common::extensions::Readable;
use crate::Result;

#[derive(Default, Debug, TryFromPrimitive)]
#[repr(u16)]
/// Location where the data referenced in the tag block is found.
pub enum TagSectionType {
    #[default]
    /// Inside the tag header (metadata)
    Header,
    /// Inside the main parent tag.
    TagData,
    /// Inside resource child tag.
    ResourceData,
    /// Inside the "external" resource (for instance, bitmaps or havok data)
    ActualResource,
}

#[derive(Default, Debug)]
/// Tag data metadata block containing data on where the binary section is located.
pub struct TagDataBlock {
    /// The size of the data block entry in bytes.
    pub entry_size: u32,
    /// How many unused bytes come before the offset.
    padding: u16,
    /// Where the data block is stored.
    pub section_type: TagSectionType,
    /// Offset of where the data is stored from the start of the tag file.
    pub offset: u64,
}

impl Readable for TagDataBlock {
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead,
    {
        self.entry_size = reader.read_u32::<LE>()?;
        self.padding = reader.read_u16::<LE>()?;
        self.section_type = TagSectionType::try_from(reader.read_u16::<LE>()?)
            .map_err(|e| Error::TagError(TagError::InvalidTagSection(e)))?;
        self.offset = reader.read_u64::<LE>()?;
        Ok(())
    }
}
