//! Main zoneset structure.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use super::{instance_tag::TagZonesetInstanceHeader, tag::TagZonesetTag};
use crate::common::extensions::{BufReaderExt, Readable};
use crate::Result;

#[derive(Default, Debug)]
/// Main zoneset instance.
pub(crate) struct TagZoneset {
    /// Header containing info on how many tags, footers, and parents to read.
    header: TagZonesetInstanceHeader,
    /// List of tags with the size of `tag_count`.
    tags: Vec<TagZonesetTag>,
    /// List of footer tags with the size of `footer_count`.
    footer_tags: Vec<TagZonesetTag>,
    /// List of 32-bit integers, unknown use.
    parents: Vec<i32>,
}

impl Readable for TagZoneset {
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead + BufReaderExt,
    {
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
