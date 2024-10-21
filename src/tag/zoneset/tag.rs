//! Zoneset tag containing its name and reference ID.

use crate::common::{errors::Error, extensions::Readable};
use byteorder::{ReadBytesExt, LE};
use std::io::BufRead;
#[derive(Default, Debug)]
pub(super) struct TagZonesetTag {
    /// Global ID of the tag.
    global_id: i32,
    /// ID of the zoneset name string.
    string_id: i32,
}

impl Readable for TagZonesetTag {
    fn read<R>(&mut self, reader: &mut R) -> Result<(), Error>
    where
        R: BufRead,
    {
        self.global_id = reader.read_i32::<LE>()?;
        self.string_id = reader.read_i32::<LE>()?;
        Ok(())
    }
}
