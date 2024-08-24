//! Tag datablock specifying the section for tag structs.

use byteorder::{ReadBytesExt, LE};
use num_enum::TryFromPrimitive;
use std::io::Read;

#[derive(Default, Debug, TryFromPrimitive)]
#[repr(u16)]
/// Location where the data referenced in the tag block is found.
pub enum TagSectionType {
    #[default]
    Header,
    TagData,
    ResourceData,
    ActualResource,
}

#[derive(Default, Debug)]
/// Tag data metadata block containing data on where the binary section is located.
pub struct TagDataBlock {
    pub entry_size: u32,
    pub pad: u16,
    pub section_type: TagSectionType,
    pub offset: u64,
}

impl TagDataBlock {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.entry_size = reader.read_u32::<LE>()?;
        self.pad = reader.read_u16::<LE>()?;
        self.section_type = TagSectionType::try_from(reader.read_u16::<LE>()?).unwrap();
        self.offset = reader.read_u64::<LE>()?;
        Ok(())
    }
}
