//! Tag Header containing info on the layout of the tag file.

pub struct TagHeader {
    pub magic: String,
    pub version: u32,
    pub hash: i64,
    pub checksum: i64,
    pub dependency_count: u32,
    pub datablock_count: u32,
    pub tagstruct_count: u32,
    pub data_reference_count: u32,
    pub tag_reference_count: u32,
    pub string_table_size: u32,
    pub zoneset_size: u32,
    pub unknown0: u32,
    pub header_size: u32,
    pub data_size: u32,
    pub resource_size: u32,
    pub actual_resource_size: u32,
    pub header_alignment: u8,
    pub tag_alignment: u8,
    pub resource_alignment: u8,
    pub actual_resource_alignment: u8,
    pub unknown1: u32,
}

impl TagHeader {
    pub fn read(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        self.magic
    }
}
