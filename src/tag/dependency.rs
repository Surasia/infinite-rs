//! Tag dependency structure containing info on lazy-loaded tags.

use crate::common::extensions::BufReaderExt;
use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
/// Dependency structure that can be used to search and lazy load for tags inside modules.
pub struct TagDependency {
    pub tag_group: String,
    pub name_offset: u32,
    pub asset_id: u64,
    pub global_id: i32,
    pub parent: i32,
}

impl TagDependency {
    pub fn read<R: Read + BufReaderExt>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.tag_group = reader.read_fixed_string(4)?.chars().rev().collect();
        self.name_offset = reader.read_u32::<LE>()?;
        self.asset_id = reader.read_u64::<LE>()?;
        self.global_id = reader.read_i32::<LE>()?;
        self.parent = reader.read_i32::<LE>()?;
        Ok(())
    }
}
