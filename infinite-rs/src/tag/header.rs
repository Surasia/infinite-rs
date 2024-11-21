//! Tag Header containing info on the layout of the tag file.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::errors::{Error, TagError};
use crate::Result;

const HEADER_MAGIC: u32 = 0x6873_6375; // "ucsh"
const HEADER_VERSION: i32 = 27;

#[derive(Default, Debug)]
/// Tag Header structure containing info on the layout of the tag file.
pub struct TagHeader {
    /// Has to be "ucsh" (0x68736375)
    magic: u32,
    /// Should be 27.
    /// Note: this is also the tag version from Halo 5!
    version: i32,
    /// Secondary GUID to identify the root structure.
    root_struct_guid: i64,
    /// Checksum generated from unknown algorithm
    checksum: i64,
    /// Number of tags required to load tag.
    pub(super) dependency_count: u32,
    /// Number of datablocks that exist within tag (offsets, sections etc).
    pub(super) datablock_count: u32,
    /// Number of tag struct definitions that make up the actual structure of the tag.
    pub(super) tagstruct_count: u32,
    /// Number of "external" data references (to other tags) in tag.
    pub(super) data_reference_count: u32,
    /// Number of internal references to structures.
    pub(super) tag_reference_count: u32,
    /// Size in bytes of string table inside tag.
    /// Unused after Halo 5.
    string_table_size: u32,
    /// Size in bytes of "zoneset" section of tag.
    /// Unknown use.
    zoneset_size: u32,
    /// Unknown. Possibly used to split something in memory.
    unknown: u32,
    /// Size of the header and the fields read by it (dependencies, datablocks, etc.).
    /// Important as sometimes the offset after reading those fields does not match up to where tag data starts.
    /// Might be some sort of internal padding measure.
    pub(crate) header_size: u32,
    /// Size of actual data in tag, referenced in tag structs.
    data_size: u32,
    /// Size of resource in tag (after data!)
    resource_size: u32,
    /// Size of "external" data, for instance Havok data.
    actual_resource_size: u32,
    /// Power of 2 to align the header to.
    header_alignment: u8,
    /// Power of 2 to align the tag data to.
    tag_alignment: u8,
    /// Power of 2 to align resource data to.
    resource_alignment: u8,
    /// Power of 2 to align actual resource to.
    actual_resource_alignment: u8,
    /// Unknown if this is consistent: Indicates if the file is a resource.
    pub is_resource: bool,
}

impl TagHeader {
    /// Reads the tag header from the given reader implementing [`BufRead`].
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements [`BufRead`] from which to read the data.
    ///
    /// # Returns
    ///
    /// This function will return an error if:
    /// * The magic string is not "ucsh"
    /// * The version is less than or equal to 17
    /// * Any I/O error occurs while reading
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.magic = reader.read_u32::<LE>()?;
        if self.magic != HEADER_MAGIC {
            return Err(Error::TagError(TagError::IncorrectMagic(self.magic)));
        }

        self.version = reader.read_i32::<LE>()?;
        if self.version != HEADER_VERSION {
            return Err(Error::TagError(TagError::IncorrectVersion(self.version)));
        }

        self.root_struct_guid = reader.read_i64::<LE>()?;
        self.checksum = reader.read_i64::<LE>()?;
        self.dependency_count = reader.read_u32::<LE>()?;
        self.datablock_count = reader.read_u32::<LE>()?;
        self.tagstruct_count = reader.read_u32::<LE>()?;
        self.data_reference_count = reader.read_u32::<LE>()?;
        self.tag_reference_count = reader.read_u32::<LE>()?;
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
