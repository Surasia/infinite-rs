//! Module Header containing info on the layout of the module file.

use byteorder::{ReadBytesExt, LE};
use num_enum::TryFromPrimitive;
use std::{fs::File, io::BufReader};

use crate::common::errors::{Error, ModuleError};
use crate::Result;

const HEADER_MAGIC: u32 = 0x6468_6F6D; // "mohd"

#[derive(Default, Debug, PartialEq, Eq, TryFromPrimitive, PartialOrd, Ord)]
#[repr(i32)]
/// Revision number of a module file.
/// This version number determines how tags should be read.
pub enum ModuleVersion {
    /// First "technical preview" build from July 2021.
    Flight1 = 48,
    /// Second technical preview (August 2021) and release version from November 2021.
    Release = 51,
    /// Build used in the co-op campaign flight, which introduced notable changes to the module structure.
    CampaignFlight = 52,
    #[default]
    /// Builds from Season 3 and onwards.
    Season3 = 53,
}

#[derive(Default, Debug)]
/// Module Header structure containing info on the layout of the module file.
pub struct ModuleHeader {
    /// Should be "mohd" (0x64686F6D)
    magic: u32,
    /// Revision number of the module.
    /// This determines how offsets are calculated and if tag names should be read.
    pub version: ModuleVersion,
    /// Unique identifier of module.
    pub module_id: i64,
    /// Number of files in the module.
    pub file_count: u32,
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
    ///
    /// This does NOT apply for versions before [`ModuleVersion::Season3`].
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
    /// * The version is not in the valid range defined by [`ModuleVersion`]
    /// * Any I/O error occurs while reading
    pub(super) fn read(&mut self, reader: &mut BufReader<File>) -> Result<()> {
        self.magic = reader.read_u32::<LE>()?;
        if self.magic != HEADER_MAGIC {
            return Err(Error::ModuleError(ModuleError::IncorrectMagic(self.magic)));
        }
        self.version = ModuleVersion::try_from_primitive(reader.read_i32::<LE>()?)
            .map_err(ModuleError::IncorrectVersion)?;

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
        if self.version >= ModuleVersion::Release {
            reader.seek_relative(8)?; // Not needed for now.
        }
        Ok(())
    }
}
