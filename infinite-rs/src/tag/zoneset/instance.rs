//! Main zoneset structure.

use byteorder::{ReadBytesExt, LE};

use super::{instance_tag::TagZonesetInstanceHeader, tag::TagZonesetTag};
use crate::common::extensions::{BufReaderExt, Enumerable};
use crate::Result;

#[derive(Default, Debug)]
/// Main zoneset instance.
pub(crate) struct TagZoneset {
    /// Header containing info on how many tags, footers, and parents to read.
    header: TagZonesetInstanceHeader,
    /// List of tags with the size of [`tag_count`](`super::instance_tag::TagZonesetInstanceHeader::tag_count`).
    tags: Vec<TagZonesetTag>,
    /// List of footer tags with the size of [`footer_count`](`super::instance_tag::TagZonesetInstanceHeader::footer_count`).
    footer_tags: Vec<TagZonesetTag>,
    /// List of 32-bit integers, unknown use.
    parents: Vec<i32>,
}

impl Enumerable for TagZoneset {
    fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.header.read(reader)?;
        self.tags = reader.read_enumerable::<TagZonesetTag>(u64::from(self.header.tag_count))?;
        self.footer_tags =
            reader.read_enumerable::<TagZonesetTag>(u64::from(self.header.footer_count))?;
        self.parents = (0..self.header.parent_count)
            .map(|_| -> Result<i32> { Ok(reader.read_i32::<LE>()?) })
            .collect::<Result<Vec<i32>>>()?;
        Ok(())
    }
}
