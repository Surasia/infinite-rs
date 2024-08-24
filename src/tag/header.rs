//! Tag Header containing info on the layout of the tag file.

use crate::common::extensions::BufReaderExt;
use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
/// Tag Header structure containing info on the layout of the tag file.
pub struct TagHeader {
    pub magic: String,
    pub version: u32,
    pub root_struct_guid: i64,
    pub checksum: i64,
    pub dependency_count: i32,
    pub datablock_count: i32,
    pub tagstruct_count: i32,
    pub data_reference_count: i32,
    pub tag_reference_count: i32,
    pub string_table_size: u32,
    pub zoneset_size: u32,
    pub block_count: u32,
    pub header_size: u32,
    pub data_size: u32,
    pub resource_size: u32,
    pub actual_resource_size: u32,
    pub header_alignment: u8,
    pub tag_alignment: u8,
    pub resource_alignment: u8,
    pub actual_resource_alignment: u8,
    pub is_resource: bool,
}

impl TagHeader {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.magic = reader.read_fixed_string(4)?;
        if self.magic != "ucsh" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid tag magic: {}", self.magic),
            ));
        }
        self.version = reader.read_u32::<LE>()?;
        if self.version != 27 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid tag version: {}", self.version),
            ));
        }
        self.root_struct_guid = reader.read_i64::<LE>()?;
        self.checksum = reader.read_i64::<LE>()?;
        self.dependency_count = reader.read_i32::<LE>()?;
        self.datablock_count = reader.read_i32::<LE>()?;
        self.tagstruct_count = reader.read_i32::<LE>()?;
        self.data_reference_count = reader.read_i32::<LE>()?;
        self.tag_reference_count = reader.read_i32::<LE>()?;
        self.string_table_size = reader.read_u32::<LE>()?;
        self.zoneset_size = reader.read_u32::<LE>()?;
        self.block_count = reader.read_u32::<LE>()?;
        self.header_size = reader.read_u32::<LE>()?;
        self.data_size = reader.read_u32::<LE>()?;
        self.resource_size = reader.read_u32::<LE>()?;
        self.actual_resource_size = reader.read_u32::<LE>()?;
        self.header_alignment = reader.read_u8()?;
        self.tag_alignment = reader.read_u8()?;
        self.resource_alignment = reader.read_u8()?;
        self.actual_resource_alignment = reader.read_u8()?;
        self.is_resource = reader.read_u32::<LE>()? != 0;
        Ok(())
    }
}
