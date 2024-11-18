//! Main abstraction file for modules.

use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    path::Path,
};

use super::{
    block::ModuleBlockEntry,
    file::{DataOffsetType, ModuleFileEntry},
    header::ModuleHeader,
};
use crate::common::extensions::BufReaderExt;
use crate::Result;

#[derive(Default, Debug)]
/// Module structure which contains the layout of the entire module file.
pub struct ModuleFile {
    /// Information relating to how the other fields should be read.
    header: ModuleHeader,
    /// Metadata regarding compression and layout of files (tags).
    pub files: Vec<ModuleFileEntry>,
    /// Indices of resource files present in the module.
    resources: Vec<u32>,
    /// Uncompressed/compressed blocks making up a file.
    blocks: Vec<ModuleBlockEntry>,
    /// Offset in `BufReader` where file data starts.
    file_data_offset: u64,
    /// Reference to the module file buffer.
    module_file: Option<BufReader<File>>,
    /// Reference to HD1 buffer if it exists.
    hd1_file: Option<BufReader<File>>,
    /// Path stored to be reused when reading HD1 modules.
    pub file_path: String,
}

impl ModuleFile {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the module file from the given file path.
    /// This function reads the entire structure of the module file.
    /// It also calculates and stores important offsets within the file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice that holds the path to the module file.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Error` containing
    /// the I/O error if any reading operation fails.
    pub fn read(&mut self, file_path: String) -> Result<()> {
        if file_path.contains("deploy/ds") {
            println!("WARNING: Loading {{ds}} module, might contain stub tags!");
        }
        let file = File::open(Path::new(&file_path))?;
        self.file_path = file_path;
        let mut reader = BufReader::new(file);

        self.header.read(&mut reader)?;
        self.open_hd1()?;

        self.files =
            reader.read_enumerable::<ModuleFileEntry>(u64::from(self.header.file_count))?;
        self.resources = (0..self.header.resource_count)
            .map(|_| -> Result<u32> { Ok(reader.read_u32::<LE>()?) })
            .collect::<Result<Vec<_>>>()?;
        self.blocks =
            reader.read_enumerable::<ModuleBlockEntry>(u64::from(self.header.block_count))?;

        // Align to 0x?????000
        let stream_position = reader.stream_position()?;
        reader.seek(SeekFrom::Start((stream_position / 0x1000 + 1) * 0x1000))?;
        self.file_data_offset = reader.stream_position()?;
        self.module_file = Some(reader);
        Ok(())
    }

    fn open_hd1(&mut self) -> Result<()> {
        if self.header.hd1_delta != 0 {
            let filepath = format!("{}_hd1", &self.file_path);
            let hd1 = Path::new(&filepath);
            if hd1.exists() {
                let file = File::open(hd1)?;
                self.hd1_file = Some(BufReader::new(file));
            }
        }
        Ok(())
    }

    /// Reads a specific tag from the module file.
    ///
    /// This function reads a specific tag from the module file based on the provided index.
    /// It checks if the tag is not a resource tag (indicated by a `tag_id` of -1) and then reads
    /// the tag data from the module file. It also utilizes the HD1 stream if the file entry has
    /// the flag set for it and the stream is loaded.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the file entry to read the tag from. This index corresponds to
    ///             the position of the file entry in the `files` vector.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Error` containing
    /// the I/O error if any reading operation fails.
    pub fn read_tag(&mut self, index: u32) -> Result<()> {
        let file = &mut self.files[index as usize];
        if file.tag_id == -1 {
            return Ok(()); // Early return for resources, some tags are simply not valid.
        }
        if file.data_offset_flags.contains(DataOffsetType::USE_HD1) {
            if let Some(ref mut module_file) = self.hd1_file {
                let offset = self.file_data_offset - self.header.hd1_delta;
                file.read_tag(module_file, offset, &self.blocks)?;
            }
        } else if let Some(ref mut module_file) = self.module_file {
            file.read_tag(module_file, self.file_data_offset, &self.blocks)?;
        }
        Ok(())
    }

    /// Searches for the index of the tag given the `global_id`.
    ///
    /// This function searches for the index of a tag in the `files` vector using the provided
    /// `global_id`. If the tag is found, it reads the tag using the `read_tag` function and
    /// stores it in the index.
    ///
    /// # Arguments
    ///
    /// * `global_id` - The global tag ID of the file to find. This ID is used to identify the
    ///                 specific tag within the module file.
    ///
    /// # Returns
    ///
    /// Returns the index of the file if successful, wrapped in `Some(usize)`. If the tag is not
    /// found, it returns `None`. Any I/O error encountered during the operation is also returned
    /// if it occurs.
    pub fn read_tag_from_id(&mut self, global_id: i32) -> Result<Option<usize>> {
        if let Some(index) = self.files.iter().position(|file| file.tag_id == global_id) {
            self.read_tag(u32::try_from(index)?)?;
            Ok(Some(index))
        } else {
            Ok(None)
        }
    }

    /// Reads the resources referenced by a module entry.
    ///
    /// This function reads the resources referenced by a specific module entry. It retrieves
    /// the resources based on the provided index and returns them as a vector of references
    /// to `ModuleFileEntry`.
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the file to read the resources of. This index corresponds to the
    ///             position of the file entry in the `files` vector.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Vec<&ModuleFileEntry>)` if the read operation is successful, containing a
    /// vector of references to `ModuleFileEntry`. If the requested resource wasn't found in
    /// the module, an `anyhow::Error` is returned.
    #[allow(clippy::cast_sign_loss)]
    pub fn read_resources(&mut self, index: u32) -> Result<Vec<&ModuleFileEntry>> {
        let entry = &self.files[index as usize];
        let mut resources = Vec::with_capacity(entry.resource_count as usize);
        for i in entry.resource_index..entry.resource_index + entry.resource_count {
            resources.push(&self.files[self.resources[i as usize] as usize]);
        }
        Ok(resources)
    }
}
