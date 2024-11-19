//! Main abstraction file for tags.

use std::io::{BufRead, Seek, SeekFrom};

use super::{
    data_reference::TagDataReference,
    datablock::TagDataBlock,
    dependency::TagDependency,
    header::TagHeader,
    reference::TagReference,
    structure::TagStruct,
    zoneset::{header::TagZonesetHeader, instance::TagZoneset},
};
use crate::common::extensions::{BufReaderExt, Readable};
use crate::Result;

#[derive(Default, Debug)]
/// Tag structure containing structure of entire tag file.
pub struct TagFile {
    /// Header containing info on how to read other parts of the file.
    pub header: TagHeader,
    /// Tags that are referenced by this tag and that need to be lazy loaded.
    pub dependencies: Vec<TagDependency>,
    /// Blocks making up the entire tag (Internal and External)
    pub datablock_definitions: Vec<TagDataBlock>,
    /// Internal structure units of the tag.
    pub struct_definitions: Vec<TagStruct>,
    /// References to external data from the tag.
    pub data_references: Vec<TagDataReference>,
    /// Tags that are referenced by this tag inside the module.
    pub tag_references: Vec<TagReference>,
    /// Zoneset header for reading the other zonesets.
    pub(crate) zoneset_header: TagZonesetHeader,
    /// Zonesets, unknown what these are.
    pub(crate) zonesets: Vec<TagZoneset>,
}

impl Readable for TagFile {
    /// Reads the tag fike from the given readers implementing `BufRead`, `BufReaderExt` and Seek.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead + BufReaderExt + Seek` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Error` if an I/O error occurs
    /// or if the header data is invalid.
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead + BufReaderExt + Seek,
    {
        self.header.read(reader)?;
        self.dependencies =
            reader.read_enumerable::<TagDependency>(u64::from(self.header.dependency_count))?;

        self.datablock_definitions =
            reader.read_enumerable::<TagDataBlock>(u64::from(self.header.datablock_count))?;

        self.struct_definitions =
            reader.read_enumerable::<TagStruct>(u64::from(self.header.tagstruct_count))?;

        self.data_references = reader
            .read_enumerable::<TagDataReference>(u64::from(self.header.data_reference_count))?;

        self.tag_references =
            reader.read_enumerable::<TagReference>(u64::from(self.header.tag_reference_count))?;

        self.zoneset_header.read(reader)?;

        self.zonesets =
            reader.read_enumerable::<TagZoneset>(u64::from(self.zoneset_header.zoneset_count))?;

        // Ensure that tag data starts where it is supposed to.
        reader.seek(SeekFrom::Start(u64::from(self.header.header_size)))?;
        Ok(())
    }
}

impl TagFile {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}
