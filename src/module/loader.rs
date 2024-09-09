//! Main abstraction file for modules.

use super::{block::ModuleBlockEntry, file::ModuleFileEntry, header::ModuleHeader};
use anyhow::Result;
use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

#[derive(Default, Debug)]
/// Module structure which contains the layout of the entire module file. Also stores file_path for re-use in read_tag.
pub struct ModuleFile {
    /// Info relating to how the other fields should be read.
    pub header: ModuleHeader,
    /// Metadata regarding compression and layout of files (tags)
    pub files: Vec<ModuleFileEntry>,
    /// String slice of file names seperated by a null terminator.
    /// Does not exist after module version 52.
    pub string_list: Vec<u8>,
    /// Indices (?) of resource files present in module.
    pub resources: Vec<u32>,
    /// Uncompressed/compressed blocks making up a file.
    pub blocks: Vec<ModuleBlockEntry>,
    /// Offset in BufReader where file data starts.
    pub file_data_offset: u64,
    /// Path stored to be re-used when reading tags.
    file_path: String,
}

impl ModuleFile {
    /// Allocate new ModuleFile and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the module file from the given file path.
    /// This function reads the entire structure of the module file.
    /// It also calculates and stores important offsets within the file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the module file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails/header checks fail.
    pub fn read(&mut self, file_path: String) -> Result<()> {
        let file = File::open(Path::new(&file_path))?;
        self.file_path = file_path;
        let mut reader = BufReader::new(file);

        self.header.read(&mut reader)?;

        self.files = (0..self.header.file_count)
            .map(|_| {
                let mut entry = ModuleFileEntry::new();
                entry.read(&mut reader).unwrap();
                entry
            })
            .collect();

        if self.header.strings_size != 0 {
            self.string_list = Vec::with_capacity(self.header.strings_size as usize);
            reader.read_exact(&mut self.string_list)?;
        }

        self.resources = (0..self.header.resource_count)
            .map(|_| reader.read_u32::<LE>().unwrap())
            .collect();

        self.blocks = (0..self.header.block_count)
            .map(|_| {
                let mut block = ModuleBlockEntry::new();
                block.read(&mut reader).unwrap();
                block
            })
            .collect();

        // Align to 0x?????000
        let stream_position = reader.stream_position()?;
        reader.seek(SeekFrom::Start((stream_position / 0x1000 + 1) * 0x1000))?;

        self.file_data_offset = reader.stream_position()?;
        Ok(())
    }

    /// Reads a specific tag from the module file.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the file entry to read the tag from
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read_tag(&mut self, index: usize) -> std::io::Result<()> {
        // -1 indicates a resource tag.
        if self.files[index].tag_id != -1 {
            self.files[index].read_tag(&self.file_path, self.file_data_offset, &self.blocks)?;
        }
        Ok(())
    }

    /// Searches for the index of the tag given the global_id.
    /// Reads the tag using read_tag and stores it in the index.
    ///
    /// # Arguments
    ///
    /// * `global_id` - The global tag ID of the file to find.
    ///
    /// # Returns
    ///
    /// Returns the index of the file if successful, 0 if it fails. Any I/O Error is also returned if it occurs.
    pub fn read_tag_from_id(&mut self, global_id: i32) -> std::io::Result<Option<usize>> {
        if let Some(index) = self.files.iter().position(|file| file.tag_id == global_id) {
            self.read_tag(index)?;
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }
}
