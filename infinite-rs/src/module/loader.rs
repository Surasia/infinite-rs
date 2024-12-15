//! Main abstraction file for modules.

use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    path::Path,
    ptr::eq,
};

use super::{
    block::ModuleBlockEntry,
    file::{DataOffsetType, ModuleFileEntry},
    header::{ModuleHeader, ModuleVersion},
};
use crate::Result;
use crate::{
    common::{errors::TagError, extensions::BufReaderExt},
    Error,
};

#[derive(Default, Debug)]
/// Module structure which contains the layout of the entire module file.
pub struct ModuleFile {
    /// Information relating to how the other fields should be read.
    pub header: ModuleHeader,
    /// Metadata regarding compression and layout of files (tags).
    pub files: Vec<ModuleFileEntry>,
    /// Indices of resource files present in the module.
    pub resource_indices: Vec<u32>,
    /// Uncompressed/compressed blocks making up a file.
    blocks: Vec<ModuleBlockEntry>,
    /// Offset in [`BufReader`] where file data starts.
    file_data_offset: u64,
    /// Reference to the module file buffer.
    module_file: Option<BufReader<File>>,
    /// Reference to HD1 buffer if it exists.
    hd1_file: Option<BufReader<File>>,
    /// Whether to use the HD1 module or not.
    pub use_hd1: bool,
}

impl ModuleFile {
    /// Instantiates a default [`ModuleFile`] object.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path<T: AsRef<Path>>(file_path: T) -> Result<Self> {
        let mut module = Self::default();
        module.read(file_path)?;
        Ok(module)
    }

    /// Reads the module file from the given file path.
    /// This function reads the entire structure of the module file.
    /// It also calculates and stores important offsets within the file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - A reference to a type that implements [`Path`] that holds the path to the module file.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an [`Error`](`crate::Error`) containing
    /// the I/O error if any reading operation fails.
    pub fn read<T: AsRef<Path>>(&mut self, file_path: T) -> Result<()> {
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);

        self.header.read(&mut reader)?;
        self.open_hd1(file_path)?;

        for _ in 0..self.header.file_count {
            let mut file = ModuleFileEntry::default();
            file.read(&mut reader, &self.header.version)?;
            self.files.push(file);
        }
        // Read strings contained in the file. A stringlist only exists in files before Season 3.
        // Each entry is separated by a null terminator, and files specify their offset themselves
        // in no particular order, so we cannot pre-read and just index into them.
        //
        // For files from modules that do not contain strings, we get it from the `get_tag_path` function.
        let strings_offset = reader.stream_position()?;
        if self.header.version <= ModuleVersion::CampaignFlight {
            for file in &mut self.files {
                reader.seek(SeekFrom::Start(
                    strings_offset + u64::from(file.name_offset),
                ))?;
                file.tag_name = reader.read_null_terminated_string()?;
            }
        } else {
            let tag_paths: Vec<String> = (0..self.files.len())
                .map(|i| self.get_tag_path(i, 0))
                .collect::<Result<Vec<_>>>()?;

            for (file, tag_path) in self.files.iter_mut().zip(tag_paths) {
                file.tag_name = tag_path;
            }
        }

        self.resource_indices = (0..self.header.resource_count)
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

    fn open_hd1<T: AsRef<Path>>(&mut self, file_path: T) -> Result<()> {
        if self.header.hd1_delta != 0 {
            let hd1 = file_path.as_ref().join("_hd1");
            if hd1.exists() {
                self.use_hd1 = true;
                let file = File::open(hd1)?;
                self.hd1_file = Some(BufReader::new(file));
            }
        }
        Ok(())
    }

    fn get_tag_path(&self, index: usize, depth: usize) -> Result<String> {
        if depth > 3 {
            return Err(Error::TagError(TagError::RecursionDepth));
        }
        let file = &self.files[index];
        if file.tag_name == "-1" && file.parent_index != -1 {
            let parent = &self.files[usize::try_from(file.parent_index)?];
            let child_index = self.resource_indices[usize::try_from(parent.resource_index)?
                ..usize::try_from(parent.resource_index)?
                    + usize::try_from(parent.resource_count)?]
                .iter()
                .map(|&i| &self.files[i as usize])
                .take_while(|&item| !eq(item, file))
                .count();

            if parent.tag_name == "-1" {
                let parent_path =
                    self.get_tag_path(usize::try_from(file.parent_index)?, depth + 1)?;
                Ok(format!("{parent_path}[{child_index}:resource]"))
            } else {
                Ok(format!(
                    "{}/{}.{}[{}:resource]",
                    parent.tag_group, parent.tag_name, parent.tag_group, child_index
                ))
            }
        } else if file.tag_id != -1 {
            Ok(format!(
                "{}/{}.{}",
                file.tag_group, file.tag_id, file.tag_group
            ))
        } else {
            Ok(file.tag_id.to_string())
        }
    }

    /// Reads a specific tag from the module file.
    ///
    /// This function reads a specific tag from the module file based on the provided index.
    /// It also utilizes the HD1 stream if the file entry has the flag set for it and the stream is loaded, and returns `None` if the tag offset is invalid.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the file entry to read the tag from. This index corresponds to
    ///             the position of the file entry in the [`files`](`ModuleFile::files`) vector.
    ///
    /// # Returns
    ///
    /// Returns `Some(i32))` containing the [`tag_id`](`ModuleFileEntry::tag_id`) of the file if the read operation is successful, or an [`Error`](`crate::Error`), a [`None`] if the file was not read (if tag offset is specified as invalid) or the containing the I/O error if any reading operation fails.
    pub fn read_tag(&mut self, index: u32) -> Result<Option<i32>> {
        let file = &mut self.files[index as usize];
        if file.data_offset_flags.contains(DataOffsetType::INVALID) {
            return Ok(None);
        }
        if file.data_offset_flags.contains(DataOffsetType::USE_HD1) {
            if let Some(ref mut module_file) = self.hd1_file {
                let mut offset = self.file_data_offset - self.header.hd1_delta;
                if self.header.version <= ModuleVersion::Season3 {
                    offset = self.header.hd1_delta;
                }
                file.read_tag(module_file, offset, &self.blocks, &self.header.version)?;
            } else {
                return Ok(None);
            }
        } else if let Some(ref mut module_file) = self.module_file {
            file.read_tag(
                module_file,
                self.file_data_offset,
                &self.blocks,
                &self.header.version,
            )?;
        }
        Ok(Some(file.tag_id))
    }

    /// Searches for the index of the tag given the `global_id`.
    ///
    /// This function searches for the index of a tag in the [`files`](`ModuleFile::files`) vector using the provided
    /// `global_id`. If the tag is found, it reads the tag using the [`read_tag`](`ModuleFile::read_tag`) function and
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
    /// found or couldn't be read, it returns [`None`]. Any I/O error encountered during the operation is also returned
    /// if it occurs.
    pub fn read_tag_from_id(&mut self, global_id: i32) -> Result<Option<usize>> {
        if let Some(index) = self.files.iter().position(|file| file.tag_id == global_id) {
            let has_read = self.read_tag(u32::try_from(index)?)?;
            if has_read.is_some() {
                Ok(Some(index))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
