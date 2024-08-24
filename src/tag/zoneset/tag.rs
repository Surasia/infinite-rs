use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
pub struct TagZonesetTag {
    pub global_id: i32,
    pub string_id: i32,
}

impl TagZonesetTag {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.global_id = reader.read_i32::<LE>()?;
        self.string_id = reader.read_i32::<LE>()?;
        Ok(())
    }
}
