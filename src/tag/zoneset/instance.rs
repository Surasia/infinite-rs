//! Main zoneset structure.

use super::{instance_tag::TagZonesetInstanceHeader, tag::TagZonesetTag};
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

#[derive(Default, Debug)]
/// Main zoneset instance.
pub struct TagZoneset {
    /// Header containing info on how many tags, footers and parents to read.
    pub header: TagZonesetInstanceHeader,
    /// Tag list with the size of tag_count.
    pub tags: Vec<TagZonesetTag>,
    /// Tag list with the size of footer_count.
    pub footer_tags: Vec<TagZonesetTag>,
    /// List of 32 bit integers, unknown use.
    pub parents: Vec<i32>,
}

impl TagZoneset {
    /// Allocate new TagZoneset and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the tag zoneset from the given readers implementing "BufRead".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read<R: BufRead>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.header.read(reader)?;
        self.tags = (0..self.header.tag_count)
            .map(|_| {
                let mut tag = TagZonesetTag::new();
                tag.read(reader).unwrap();
                tag
            })
            .collect();
        self.footer_tags = (0..self.header.footer_count)
            .map(|_| {
                let mut tag = TagZonesetTag::new();
                tag.read(reader).unwrap();
                tag
            })
            .collect();

        self.parents = (0..self.header.parent_count)
            .map(|_| reader.read_i32::<LE>().unwrap())
            .collect();
        Ok(())
    }
}
