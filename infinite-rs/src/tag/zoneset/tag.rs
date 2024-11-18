//! Zoneset tag containing its name and reference ID.

use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;

use crate::common::extensions::Readable;
use crate::Result;

#[derive(Default, Debug)]
pub(super) struct TagZonesetTag {
    /// Global ID of the tag.
    global_id: i32,
    /// ID of the zoneset name string.
    string_id: i32,
}

impl Readable for TagZonesetTag {
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead,
    {
        self.global_id = reader.read_i32::<LE>()?;
        self.string_id = reader.read_i32::<LE>()?;
        Ok(())
    }
}
