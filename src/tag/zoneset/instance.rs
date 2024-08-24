use super::{instance_tag::TagZonesetInstanceHeader, tag::TagZonesetTag};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;

#[derive(Default, Debug)]
pub struct TagZoneset {
    pub header: TagZonesetInstanceHeader,
    pub tags: Vec<TagZonesetTag>,
    pub footer_tags: Vec<TagZonesetTag>,
    pub parents: Vec<i32>,
}

impl TagZoneset {
    pub fn read<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.header.read(reader)?;
        self.tags = (0..self.header.tag_count)
            .map(|_| {
                let mut tag = TagZonesetTag::default();
                tag.read(reader).unwrap();
                tag
            })
            .collect();
        self.footer_tags = (0..self.header.footer_count)
            .map(|_| {
                let mut tag = TagZonesetTag::default();
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
