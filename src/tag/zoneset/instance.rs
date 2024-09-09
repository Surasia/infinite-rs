//! Main zoneset structure.

use super::{instance_tag::TagZonesetInstanceHeader, tag::TagZonesetTag};
use crate::common::extensions::{BufReaderExt, Readable};
use anyhow::Result;
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

impl Readable for TagZoneset {
    /// Reads the tag zoneset from the given readers implementing "BufRead".
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read<R: BufRead + BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.header.read(reader)?;
        self.tags = reader.read_enumerable::<TagZonesetTag>(self.header.tag_count as usize)?;
        self.footer_tags =
            reader.read_enumerable::<TagZonesetTag>(self.header.footer_count as usize)?;
        self.parents = (0..self.header.parent_count)
            .map(|_| reader.read_i32::<LE>().unwrap())
            .collect();
        Ok(())
    }
}
