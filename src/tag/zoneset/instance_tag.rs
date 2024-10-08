//! Instance of a zoneset tag that comes after the tag header.

use crate::common::extensions::Readable;
use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

#[derive(Default, Debug)]
/// An instance of a zoneset dictating how many tags to read.
pub struct TagZonesetInstanceHeader {
    /// The name of the zoneset that this tag belongs to.
    pub string_id: i32,
    /// Number of tags loaded for this zoneset.
    pub tag_count: u32,
    /// The count of 4 byte items (unknown)?
    pub parent_count: u32,
    /// Same as tag count (unknown?)
    pub footer_count: u32,
}

impl Readable for TagZonesetInstanceHeader {
    /// Allocate new TagZonesetInstanceHeader and set it to default values.
    /// Reads the tag zoneset instance header from the given readers implementing "BufRead".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Err` if an I/O error occurs
    /// or if the header data is invalid.
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.string_id = reader.read_i32::<LE>()?;
        self.tag_count = reader.read_u32::<LE>()?;
        self.parent_count = reader.read_u32::<LE>()?;
        self.footer_count = reader.read_u32::<LE>()?;
        Ok(())
    }
}
