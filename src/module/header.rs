//! Module Header containing info on the layout of the module file.

use crate::common::extensions::BufReaderExt;
use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{self, BufReader},
};

#[derive(Default, Debug)]
/// Module Header structure containing info on the layout of the module file.
/// Version 52+.
pub struct ModuleHeader {
    /// Should be "mohd".
    pub magic: String,
    /// Flight 1: 48 /
    /// Flight 2: 51 & Retail /
    /// Season 3+: 52
    pub version: i32,
    /// Unique identifier. (not a hash?)
    pub module_id: i64,
    /// Amount of files in module.
    pub file_count: u32,
    /// Unknown: not in all modules.
    pub manifest0_count: u32,
    /// Unknown: present in most modules.
    pub manifest1_count: u32,
    /// Unknown: not present in any modules.
    pub manifest2_count: u32,
    /// Index of the first resource entry (file_count - resource_count)
    pub resource_index: i32,
    /// Total size in bytes of the string table.
    pub strings_size: u32,
    /// Number of resource files.
    pub resource_count: u32,
    /// Number of data blocks.
    pub block_count: u32,
    /// Same between modules, changes per build?
    pub build_version: u64,
    /// If non-zero, requires hd1 file.
    pub hd1_delta: u64,
    /// Total size of packed data in module.
    /// Both compressed and uncompressed
    /// Starts after files, blocks, resources have been read.
    pub data_size: u64,
}

impl ModuleHeader {
    /// Allocate new ModuleHeader and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the module header from the given buffered reader.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the header is successfully read, or an `Err` if an I/O error occurs
    /// or if the header data is invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The magic string is not "mohd"
    /// * The version is less than or equal to 0x34
    /// * Any I/O error occurs while reading
    pub fn read(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        self.magic = reader.read_fixed_string(4)?;
        if self.magic != "mohd" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid magic: {}", self.magic),
            ));
        }

        self.version = reader.read_i32::<LE>()?;
        if self.version <= 0x34 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid module version: {}", self.version),
            ));
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
        Ok(())
    }
}
