//! Zoneset header containing info on the entirety of the zoneset structure.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Enumerable;
use crate::Result;

#[derive(Default, Debug)]
/// Zoneset header used to read tags.
pub(crate) struct TagZonesetHeader {
    /// Identifier for the name of the zoneset that this tag belongs to.
    string_id: i32,
    /// Number of zonesets to load for this tag.
    pub(crate) zoneset_count: u32,
    /// Unknown: 4 byte items?
    footer_count: u32,
    /// Number of parent tags to load for this zoneset.
    parents: u32,
}

impl Enumerable for TagZonesetHeader {
    fn read<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        self.string_id = reader.read_i32::<LE>()?;
        self.zoneset_count = reader.read_u32::<LE>()?;
        self.footer_count = reader.read_u32::<LE>()?;
        self.parents = reader.read_u32::<LE>()?;
        Ok(())
    }
}
