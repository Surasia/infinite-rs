//! Main abstraction file for tags.

use std::io::SeekFrom;

use super::{
    data_reference::TagDataReference, datablock::TagDataBlock, dependency::TagDependency,
    header::TagHeader, reference::TagReference, structure::TagStruct,
};
use crate::Result;
use crate::common::extensions::BufReaderExt;
use crate::module::header::ModuleVersion;

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
}

impl TagFile {
    /// Reads the tag file from the given readers implementing [`BufReaderExt`].
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements [`BufReaderExt`] from which to read the data.
    /// * `module_version` - Version of the module being read
    ///
    /// # Errors
    /// - If the reader fails to read the exact number of bytes [`ReadError`](`crate::Error::ReadError`)
    pub fn read<R: BufReaderExt>(&mut self, reader: &mut R, version: &ModuleVersion) -> Result<()> {
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

        let string_table_position = reader.stream_position()?;

        // This is only valid before Season 3.
        if version < &ModuleVersion::Season3 {
            for dep in &mut self.dependencies {
                reader.seek(SeekFrom::Start(
                    string_table_position + u64::from(dep.name_offset),
                ))?;
                dep.name = Some(reader.read_null_terminated_string()?);
            }
            for reference in &mut self.tag_references {
                reader.seek(SeekFrom::Start(
                    string_table_position + u64::from(reference.name_offset),
                ))?;
                reference.name = Some(reader.read_null_terminated_string()?);
            }
        }
        // Ensure that tag data starts where it is supposed to.
        reader.seek(SeekFrom::Start(u64::from(self.header.header_size)))?;
        Ok(())
    }
}
