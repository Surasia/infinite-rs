//! Main abstraction file for tags.

use crate::common::extensions::BufReaderExt;

use super::{
    data_reference::TagDataReference,
    datablock::TagDataBlock,
    dependency::TagDependency,
    header::TagHeader,
    reference::TagReference,
    structure::TagStruct,
    zoneset::{header::TagZonesetHeader, instance::TagZoneset},
};
use std::io::{Read, Seek, SeekFrom};

#[derive(Default, Debug)]
pub struct TagFile {
    pub header: TagHeader,
    pub dependencies: Vec<TagDependency>,
    pub datablock: Vec<TagDataBlock>,
    pub structs: Vec<TagStruct>,
    pub data_references: Vec<TagDataReference>,
    pub tag_references: Vec<TagReference>,
    pub zoneset_header: TagZonesetHeader,
    pub zonesets: Vec<TagZoneset>,
}

impl TagFile {
    pub fn read<R: Read + BufReaderExt + Seek>(
        &mut self,
        mut reader: &mut R,
    ) -> std::io::Result<()> {
        self.header.read(reader)?;

        self.dependencies = (0..self.header.dependency_count as usize)
            .map(|_| {
                let mut dependency = TagDependency::default();
                dependency.read(reader).unwrap();
                dependency
            })
            .collect();

        self.datablock = (0..self.header.datablock_count as usize)
            .map(|_| {
                let mut block = TagDataBlock::default();
                block.read(reader).unwrap();
                block
            })
            .collect();

        self.structs = (0..self.header.tagstruct_count as usize)
            .map(|_| {
                let mut block = TagStruct::default();
                block.read(reader).unwrap();
                block
            })
            .collect();

        self.data_references = (0..self.header.data_reference_count as usize)
            .map(|_| {
                let mut block = TagDataReference::default();
                block.read(reader).unwrap();
                block
            })
            .collect();

        self.tag_references = (0..self.header.tag_reference_count as usize)
            .map(|_| {
                let mut block = TagReference::default();
                block.read(reader).unwrap();
                block
            })
            .collect();

        self.zoneset_header.read(&mut reader)?;
        self.zonesets = (0..self.zoneset_header.zoneset_count as usize)
            .map(|_| {
                let mut zoneset = TagZoneset::default();
                zoneset.read(reader).unwrap();
                zoneset
            })
            .collect();

        // Ensure that tag data starts where it is supposed to.
        reader.seek(SeekFrom::Start(self.header.header_size as u64))?;
        Ok(())
    }
}
