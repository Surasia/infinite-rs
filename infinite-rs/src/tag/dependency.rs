//! Tag dependency structure containing info on lazy-loaded tags.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::{BufReaderExt, Readable};
use crate::Result;

#[derive(Default, Debug)]
/// Dependency structure that can be used to search and lazy load for tags inside modules.
pub struct TagDependency {
    /// 4 byte-long string for tag group, stored as big endian
    /// Example:
    /// * bitm: Bitmap
    /// * mat: Material
    pub tag_group: String,
    /// Offset in global string table where the name of the tag is stored.
    /// This is no longer valid as of module version 52
    name_offset: u32,
    /// Higher significant of `MurmurHash3_x86_64` 128 bit hash of raw tag path (before cache compilation).
    pub asset_id: u64,
    /// `MurmurHash3_x86_64` 32 bit hash of tag path.
    /// Referred to in-memory as "global tag id"
    /// Is set to -1 if file is resource.
    pub tag_id: i32,
    /// Index of parent in module.
    pub parent_index: i32,
}

impl Readable for TagDependency {
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead + BufReaderExt,
    {
        self.tag_group = reader.read_fixed_string(4)?.chars().rev().collect(); // Reverse string
        self.name_offset = reader.read_u32::<LE>()?;
        self.asset_id = reader.read_u64::<LE>()?;
        self.tag_id = reader.read_i32::<LE>()?;
        self.parent_index = reader.read_i32::<LE>()?;
        Ok(())
    }
}
