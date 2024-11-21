//! Instance of a zoneset tag that comes after the tag header.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Enumerable;
use crate::Result;

#[derive(Default, Debug)]
/// An instance of a zoneset dictating how many tags to read.
pub(crate) struct TagZonesetInstanceHeader {
    /// The ID of the zoneset that this tag belongs to.
    string_id: i32,
    /// Number of tags loaded for this zoneset.
    pub(super) tag_count: u32,
    /// The count of 4-byte items (unknown).
    pub(super) parent_count: u32,
    /// Same as tag count (unknown).
    pub(super) footer_count: u32,
}

impl Enumerable for TagZonesetInstanceHeader {
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.string_id = reader.read_i32::<LE>()?;
        self.tag_count = reader.read_u32::<LE>()?;
        self.parent_count = reader.read_u32::<LE>()?;
        self.footer_count = reader.read_u32::<LE>()?;
        Ok(())
    }
}
