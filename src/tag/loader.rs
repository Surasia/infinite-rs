//! Main abstraction file for tags.

use anyhow::Result;

use super::{
    data_reference::TagDataReference,
    datablock::TagDataBlock,
    dependency::TagDependency,
    header::TagHeader,
    reference::TagReference,
    structure::TagStruct,
    types::structs::{
        cmsw::CoatingSwatchPODTag, jssc::JsonSourceFileTagDefinition, luas::LuaScriptTagDefinition,
    },
    zoneset::{header::TagZonesetHeader, instance::TagZoneset},
};
use crate::common::extensions::{BufReaderExt, Readable};
use std::any::Any;
use std::io::{BufRead, Seek, SeekFrom};

#[derive(Default, Debug)]
/// Tag structure containing structure of entire tag file.
pub struct TagFile {
    /// Header containing info on how to read other parts of the file.
    pub header: TagHeader,
    /// Tags that are referenced by this tag and that need to be lazy loaded.
    pub dependencies: Vec<TagDependency>,
    /// Blocks making up the entire tag (Internal and External)
    pub datablock: Vec<TagDataBlock>,
    /// Internal structure units of the tag.
    pub structs: Vec<TagStruct>,
    /// References to external data from the tag.
    pub data_references: Vec<TagDataReference>,
    /// Tags that are referenced by this tag inside the module.
    pub tag_references: Vec<TagReference>,
    /// Zoneset header for reading the other zonesets.
    pub zoneset_header: TagZonesetHeader,
    /// Zonesets, unknown what these are.
    pub zonesets: Vec<TagZoneset>,
    /// Actual structure depending on their tag group.
    /// Access this using get_struct.
    pub structure: Option<Box<dyn Any>>,
}

impl Readable for TagFile {
    /// Reads the tag fike from the given readers implementing BufRead, BufReaderExt and Seek.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead + BufReaderExt + Seek` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `(anyhow) Error` if an I/O error occurs
    /// or if the header data is invalid.
    fn read<R: BufRead + BufReaderExt + Seek>(&mut self, reader: &mut R) -> Result<()> {
        self.header.read(reader)?;
        self.dependencies =
            reader.read_enumerable::<TagDependency>(self.header.dependency_count as usize)?;

        self.datablock =
            reader.read_enumerable::<TagDataBlock>(self.header.datablock_count as usize)?;

        self.structs = reader.read_enumerable::<TagStruct>(self.header.tagstruct_count as usize)?;

        self.data_references = reader
            .read_enumerable::<TagDataReference>(self.header.data_reference_count as usize)?;

        self.tag_references =
            reader.read_enumerable::<TagReference>(self.header.tag_reference_count as usize)?;

        self.zoneset_header.read(reader)?;

        self.zonesets =
            reader.read_enumerable::<TagZoneset>(self.zoneset_header.zoneset_count as usize)?;

        // Ensure that tag data starts where it is supposed to.
        reader.seek(SeekFrom::Start(self.header.header_size as u64))?;
        Ok(())
    }
}

impl TagFile {
    /// Reads the "structure" according to a tag's group.
    ///
    /// This function creates a Box<> smart pointer depending on the type of the tag.
    /// The structure can later be obtained using get_struct.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead, BufReaderExt and Seek` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Err` if an I/O error occurs
    /// or if the header data is invalid.
    pub fn read_struct<R: BufRead + BufReaderExt + Seek>(
        &mut self,
        tag_group: &str,
        reader: &mut R,
    ) -> std::io::Result<()> {
        match tag_group {
            "jssc" => {
                let mut jssc = JsonSourceFileTagDefinition::new();
                jssc.read(reader)?;
                self.structure = Some(Box::new(jssc));
            }
            "cmsw" => {
                let mut cmsw = CoatingSwatchPODTag::new();
                cmsw.read(reader)?;
                self.structure = Some(Box::new(cmsw));
            }
            "luas" => {
                let mut luas = LuaScriptTagDefinition::new();
                luas.read(reader)?;
                self.structure = Some(Box::new(luas));
            }
            _ => (),
        }
        Ok(())
    }

    /// Get the "structure" according to a tag's group.
    ///
    /// This function returns an `Option<&T>` where `T` is the type specified when calling the function.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type to cast the structure to.
    ///
    /// # Returns
    ///
    /// Returns `Some(&T)` if the structure exists and can be cast to type T, or `None` otherwise.
    pub fn get_struct<T: 'static>(&self) -> Option<&T> {
        self.structure
            .as_ref()
            .and_then(|data| data.downcast_ref::<T>())
    }
}
