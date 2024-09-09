//! Tag Header containing info on the layout of the tag file.

use crate::common::{errors::TagError, extensions::BufReaderExt};
use anyhow::{bail, Result};
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

#[derive(Default, Debug)]
/// Tag Header structure containing info on the layout of the tag file.
pub struct TagHeader {
    /// Has to be "ucsh"
    pub magic: String,
    /// Should be 27.
    /// Note: this is also the tag version from Halo 5!
    pub version: i32,
    /// Secondary GUID to identify the root structure.
    pub root_struct_guid: i64,
    /// Checksum generated from unknown algorithm
    pub checksum: i64,
    /// Number of tags required to load tag.
    pub dependency_count: i32,
    /// Number of datablocks that exist within tag (offsets, sections etc).
    pub datablock_count: i32,
    /// Number of tag struct definitions that make up the actual structure of the tag.
    pub tagstruct_count: i32,
    /// Number of "external" data references (to other tags) in tag.
    pub data_reference_count: i32,
    /// Number of internal references to structures.
    pub tag_reference_count: i32,
    /// Size in bytes of string table inside tag.
    /// Unused after Halo 5.
    pub string_table_size: u32,
    /// Size in bytes of "zoneset" section of tag.
    /// Unknown use.
    pub zoneset_size: u32,
    /// Unknown. Possibly used to split something in memory.
    pub unknown: u32,
    /// Size of the header and the fields read by it (dependencies, datablocks, etc.).
    /// Important as sometimes the offset after reading those fields does not match up to where tag data starts.
    /// Might be some sort of internal padding measure.
    pub header_size: u32,
    /// Size of actual data in tag, referenced in tag structs.
    pub data_size: u32,
    /// Size of resource in tag (after data!)
    pub resource_size: u32,
    /// Size of "external" data, for instance Havok data.
    pub actual_resource_size: u32,
    /// Power of 2 to align the header to.
    pub header_alignment: u8,
    /// Power of 2 to align the tag data to.
    pub tag_alignment: u8,
    /// Power of 2 to align resource data to.
    pub resource_alignment: u8,
    /// Power of 2 to align actual resource to.
    pub actual_resource_alignment: u8,
    /// Unknown if this is consistent: Indicates if the file is a resource.
    pub is_resource: bool,
}

impl TagHeader {
    /// Allocate new TagHeader and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the tag header from the given readers implementing BufRead and BufReaderExt.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead + BufReaderExt` from which to read the data.
    ///
    /// # Returns
    ///
    /// This function will return an error if:
    /// * The magic string is not "ucsh"
    /// * The version is less than or equal to 17
    /// * Any I/O error occurs while reading
    pub fn read<R: BufRead + BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.magic = reader.read_fixed_string(4)?;
        if self.magic != "ucsh" {
            bail!(TagError::IncorrectMagic(self.magic.clone()))
        }

        self.version = reader.read_i32::<LE>()?;
        if self.version != 27 {
            bail!(TagError::IncorrectVersion(self.version))
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
        self.unknown = reader.read_u32::<LE>()?;
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
