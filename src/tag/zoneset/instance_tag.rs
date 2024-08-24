use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
pub struct TagZonesetInstanceHeader {
    pub string_id: i32,
    pub tag_count: u32,
    pub parent_count: u32,
    pub footer_count: u32,
}

impl TagZonesetInstanceHeader {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.string_id = reader.read_i32::<LE>()?;
        self.tag_count = reader.read_u32::<LE>()?;
        self.parent_count = reader.read_u32::<LE>()?;
        self.footer_count = reader.read_u32::<LE>()?;
        Ok(())
    }
}
