//! Module Header containing info on the layout of the module file.

use byteorder::{ReadBytesExt, LE};
use std::{fs::File, io::BufReader};

use crate::common::errors::{Error, ModuleError};
use crate::Result;

const HEADER_MAGIC: u32 = 0x6468_6F6D; // "mohd"
const HEADER_VERSION: i32 = 0x34; // 52

#[derive(Default, Debug)]
/// Module Header structure containing info on the layout of the module file.
/// Version 53+.
pub struct ModuleHeader {
    /// Should be "mohd" (0x64686F6D)
    magic: u32,
    /// Flight 1: 48 /
    /// Flight 2: 51 & Retail /
    /// Season 3+: 52
    /// CU30+: 53
    version: i32,
    /// Unique identifier. (not a hash?)
    module_id: i64,
    /// Number of files in the module.
    pub(super) file_count: u32,
    /// Unknown: not in all modules.
    manifest0_count: u32,
    /// Unknown: present in most modules.
    manifest1_count: u32,
    /// Unknown: not present in any modules.
    manifest2_count: u32,
    /// Index of the first resource entry ([`file_count`](`ModuleHeader::file_count`) - [`resource_count`](`ModuleHeader::resource_count`)).
    resource_index: i32,
    /// Total size in bytes of the string table.
    strings_size: u32,
    /// Number of resource files.
    pub(super) resource_count: u32,
    /// Number of data blocks.
    pub(super) block_count: u32,
    /// Same between modules, changes per build?
    build_version: u64,
    /// If non-zero, requires hd1 file.
    pub(super) hd1_delta: u64,
    /// Total size of packed data in the module.
    /// Both compressed and uncompressed.
    /// Starts after files, blocks, and resources have been read.
    pub(super) data_size: u64,
}

impl ModuleHeader {
    /// Reads the module header from the given buffered reader.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a [`BufReader<File>`] from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an [`Error`] if an I/O error occurs
    /// or if the header data is invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The magic string is not "mohd"
    /// * The version is less than or equal to 0x34
    /// * Any I/O error occurs while reading
    pub(super) fn read(&mut self, reader: &mut BufReader<File>) -> Result<()> {
        self.magic = reader.read_u32::<LE>()?;
        if self.magic != HEADER_MAGIC {
            return Err(Error::ModuleError(ModuleError::IncorrectMagic(self.magic)));
        }

        self.version = reader.read_i32::<LE>()?;
        if self.version <= HEADER_VERSION {
            return Err(Error::ModuleError(ModuleError::IncorrectVersion(
                self.version,
            )));
        }

        self.module_id = reader.read_i64::<LE>()?;
        self.file_count = reader.read_u32::<LE>()?;
        self.manifest0_count = reader.read_u32::<LE>()?;
        self.manifest1_count = reader.read_u32::<LE>()?;
        self.manifest2_count = reader.read_u32::<LE>()?;
        self.resource_index = reader.read_i32::<LE>()?;
        self.strings_size = reader.read_u32::<LE>()?;
        self.resource_count = reader.read_u32::<LE>()?;
        self.block_count = reader.read_u32::<LE>()?;
        self.build_version = reader.read_u64::<LE>()?;
        self.hd1_delta = reader.read_u64::<LE>()?;
        self.data_size = reader.read_u64::<LE>()?;
        reader.seek_relative(8)?; // Not needed for now.
        Ok(())
    }
}
