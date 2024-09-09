//! Module file entry containing metadata relating to tags and functions to read them.

use super::{block::ModuleBlockEntry, kraken::decompress};
use crate::{common::extensions::BufReaderExt, tag::loader::TagFile, ModuleFile};
use anyhow::Result;
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
    path::Path,
};

bitflags! {
    #[derive(Debug, Default)]
    /// Flags for the last 2 bytes of the data offset.
    pub struct DataOffsetType : u16  {
        /// No additional HD1 module is required.
        const USE_SELF = !0;
        /// Additional HD1 module is required.
        const USE_HD1 = 0b00000001;
        /// Unknown (HD2)?
        const USE_HD2 = 0b00000010;
    }
}

bitflags! {
    #[derive(Debug, Default)]
    /// Flags that determine how a tag should be read.
    pub struct FileEntryFlags : u8  {
        /// If tag is Oodle compressed or not.
        const COMPRESSED = 0b00000001;
        /// Indicates that tag is made up of "tag blocks" which need to be joined to assemble the
        /// entire file entry.
        const HAS_BLOCKS = 0b00000010;
        /// "Raw tag" that does not contain a tag header.
        const RAW_FILE = 0b00000100;
    }
}

#[derive(Default, Debug)]
/// Module file entry structure containing metadata relating to file and required buffer sizes and offsets for the decompressor, as well as global tag ID, resource references and class.
pub struct ModuleFileEntry {
    /// Unknown, some sort of size?
    pub unknown: u8,
    /// Determine how the file should be read.
    pub flags: FileEntryFlags,
    /// Number of blocks that make up the file.
    pub block_count: u16,
    /// Index of the first block in the module.
    pub block_index: i32,
    /// Index of the first resource in the module's resource list.
    pub resource_index: i32,
    /// 4 byte-long string for tag group, stored as big endian. This determines how the rest of the tag is read.
    /// Example:
    /// * bitm: Bitmap
    /// * mat: Material
    pub tag_group: String,
    /// Offset of compressed/uncompressed data in from the start of compressed data in the module.
    pub data_offset: u64,
    /// Where the offset is located. 1 if in HD1.
    pub data_offset_flags: DataOffsetType,
    /// Size in bytes of compressed buffer in module.
    pub total_compressed_size: u32,
    /// Size in bytes of buffer to decompress into.
    pub total_uncompressed_size: u32,
    /// MurmurHash3_x86_64 32 bit hash of tag path.
    /// Referred to in-memory as "global tag id"
    /// Is set to -1 if file is resource.
    pub tag_id: i32,
    /// Size in bytes of header in decompressed buffer.
    pub uncompressed_header_size: u32,
    /// Size in bytes of actual tag data in decompressed buffer.
    pub uncompressed_tag_data_size: u32,
    /// Size in bytes of resource data in decompressed buffer.
    pub uncompressed_resource_data_size: u32,
    /// Size in bytes of "external" resource data in decompressed buffer. (for instance, havok data or bitmaps)
    pub uncompressed_actual_resource_size: u32,
    /// Power of 2 to align the header buffer to (e.g. 4 = align to a multiple of 16 bytes).
    pub header_alignment: u8,
    /// Power of 2 to align the tag data buffer to.
    pub tag_data_alignment: u8,
    /// Power of 2 to align the resource data buffer to.
    pub resource_data_alignment: u8,
    /// Power of 2 to align the actual resource data buffer to.
    pub actual_resource_data_alignment: u8,
    /// Offset where the name of the file is located in the string table.
    /// This is no longer valid as of module version 52.
    pub name_offset: u32,
    /// Used with resources to point back to the parent file. -1 = none
    pub parent_index: i32,
    /// Murmur3_x64_128 hash of (what appears to be) the original file that this file was built from.
    /// This is not always the same thing as the file stored in the module.
    /// Only verified if the HasBlocks flag is not set.
    pub asset_hash: i128,
    /// Number of resources owned by the file.
    pub resource_count: i32,
    /// Data stream containing a buffer of bytes to read/seek.
    pub data_stream: Cursor<Vec<u8>>,
    /// The actual tag file read from the contents (including header), only valid if file is not a resource.
    pub tag_info: TagFile,
    /// Indicates if file is cached (non-lazy loaded) or not.
    is_loaded: bool,
}

impl ModuleFileEntry {
    /// Allocate new ModuleFileEntry and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the metadata of a module file entry from the given reader.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read<R: BufRead + BufReaderExt + Seek>(
        &mut self,
        reader: &mut R,
    ) -> std::io::Result<()> {
        self.unknown = reader.read_u8()?;
        self.flags = FileEntryFlags::from_bits_truncate(reader.read_u8()?);
        self.block_count = reader.read_u16::<LE>()?;
        self.block_index = reader.read_i32::<LE>()?;
        self.resource_index = reader.read_i32::<LE>()?;
        self.tag_group = reader.read_fixed_string(4)?.chars().rev().collect(); // Reverse string
        self.data_offset = reader.read_u64::<LE>()? & 0x0000FFFFFFFFFFFF; // Mask last 6 bytes
        self.data_offset_flags =
            DataOffsetType::from_bits_truncate((self.data_offset >> 48) as u16); // Read first 2 bytes
        self.total_compressed_size = reader.read_u32::<LE>()?;
        self.total_uncompressed_size = reader.read_u32::<LE>()?;
        self.tag_id = reader.read_i32::<LE>()?;
        self.uncompressed_header_size = reader.read_u32::<LE>()?;
        self.uncompressed_tag_data_size = reader.read_u32::<LE>()?;
        self.uncompressed_resource_data_size = reader.read_u32::<LE>()?;
        self.uncompressed_actual_resource_size = reader.read_u32::<LE>()?;
        self.header_alignment = reader.read_u8()?;
        self.tag_data_alignment = reader.read_u8()?;
        self.resource_data_alignment = reader.read_u8()?;
        self.actual_resource_data_alignment = reader.read_u8()?;
        self.name_offset = reader.read_u32::<LE>()?;
        self.parent_index = reader.read_i32::<LE>()?;
        self.asset_hash = reader.read_i128::<LE>()?;
        self.resource_count = reader.read_i32::<LE>()?;
        reader.seek_relative(4)?; // Skip some padding?
        Ok(())
    }

    /// Reads and loads tag data from a file.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file containing tag data.
    /// * `data_offset` - Starting offset of the data in the file.
    /// * `blocks` - Metadata for data blocks.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `(anyhow) Error` containing
    /// the I/O error if any reading operation fails.
    pub fn read_tag(
        &mut self,
        file_path: &String,
        data_offset: u64,
        blocks: &[ModuleBlockEntry],
    ) -> Result<()> {
        if !self.is_loaded {
            let file = File::open(Path::new(file_path))?;
            let mut reader = BufReader::new(file);

            let file_offset = data_offset + self.data_offset;
            let mut data = vec![0u8; self.total_uncompressed_size as usize];

            if self.block_count != 0 {
                self.read_multiple_blocks(&mut reader, blocks, file_offset, &mut data)?;
            } else {
                self.read_single_block(&mut reader, self, file_offset, &mut data)?;
            }

            // .clone_from reuses resources, limiting unneeded allocations.
            let data_stream = std::io::Cursor::new(data);
            self.data_stream.clone_from(&data_stream);

            let mut buf_reader = BufReader::new(data_stream);
            self.tag_info.read(&mut buf_reader)?;
            self.tag_info
                .read_struct(&self.tag_group, &mut buf_reader)?;
            self.is_loaded = true;
        }
        Ok(())
    }

    /// Reads multiple blocks of data from the file.
    ///
    /// This function reads multiple blocks of data, which can be either compressed or uncompressed,
    /// from the file and stores them in the provided data buffer.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    /// * `blocks` - A slice of `ModuleBlockEntry` containing metadata about each block.
    /// * `file_offset` - The offset in the file where the data blocks start.
    /// * `data` - A mutable slice where the (decompressed) data will be stored.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read_multiple_blocks(
        &self,
        reader: &mut BufReader<File>,
        blocks: &[ModuleBlockEntry],
        file_offset: u64,
        data: &mut [u8],
    ) -> std::io::Result<()> {
        let first_block_index = self.block_index as usize;
        reader.seek(SeekFrom::Start(file_offset))?;

        for block in &blocks[first_block_index..(first_block_index + self.block_count as usize)] {
            if block.is_compressed {
                self.read_compressed_block(reader, block, data)?;
            } else {
                self.read_uncompressed_block(reader, block, data)?;
            }
        }
        Ok(())
    }

    /// Reads an uncompressed block of data from the file.
    ///
    /// This function reads an uncompressed block directly from the file and copies it
    /// into the appropriate section of the output buffer.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    /// * `block` - A reference to the `ModuleBlockEntry` containing metadata about the block.
    /// * `data` - A mutable slice where the uncompressed data will be stored.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read_uncompressed_block(
        &self,
        reader: &mut BufReader<File>,
        block: &ModuleBlockEntry,
        data: &mut [u8],
    ) -> std::io::Result<()> {
        reader.read_exact(
            &mut data[block.decompressed_offset as usize
                ..(block.decompressed_offset + block.compressed_size) as usize],
        )?;
        Ok(())
    }

    /// Reads and decompresses a compressed block of data.
    ///
    /// This function reads a compressed block from the file, decompresses it,
    /// and then copies the decompressed data into the appropriate section of the output buffer.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    /// * `block` - A reference to the `ModuleBlockEntry` containing metadata about the block.
    /// * `data` - A mutable slice where the decompressed data will be stored.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read_compressed_block(
        &self,
        reader: &mut BufReader<File>,
        block: &ModuleBlockEntry,
        data: &mut [u8],
    ) -> std::io::Result<()> {
        let mut compressed_data = vec![0u8; block.compressed_size as usize];
        reader.read_exact(&mut compressed_data)?;
        let mut decompressed_data = vec![0u8; block.decompressed_size as usize];
        decompress(
            compressed_data,
            &mut decompressed_data,
            block.decompressed_size as usize,
        );
        data[block.decompressed_offset as usize
            ..(block.decompressed_offset + block.decompressed_size) as usize]
            .copy_from_slice(&decompressed_data);
        Ok(())
    }

    /// Reads a single block of data from the file.
    ///
    /// This function is used when the file entry contains only one block of data.
    /// It reads the entire block, and then either copies it directly to the output
    /// if it's not compressed, or decompresses it if necessary.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    /// * `file_entry` - A reference to the `ModuleFileEntry` containing metadata about the file.
    /// * `file_offset` - The offset in the file where the data block starts.
    /// * `data` - A mutable reference to the `Vec<u8>` where the (decompressed) data will be stored.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read_single_block(
        &self,
        reader: &mut BufReader<File>,
        file_entry: &ModuleFileEntry,
        file_offset: u64,
        data: &mut Vec<u8>,
    ) -> std::io::Result<()> {
        reader.seek(SeekFrom::Start(file_offset))?;
        let compressed_size = file_entry.total_compressed_size as usize;
        let mut block = vec![0u8; compressed_size];
        reader.read_exact(&mut block)?;

        if compressed_size == file_entry.total_uncompressed_size as usize {
            data.copy_from_slice(&block);
        } else {
            decompress(block, data, file_entry.total_uncompressed_size as usize);
        }
        Ok(())
    }

    /// Reads the resources referenced by a module entry.
    ///
    /// # Arguments
    ///
    /// * `module` - A static reference to the module of where the resources reside.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read_resources(
        &self,
        module: &'static ModuleFile,
    ) -> std::io::Result<Vec<&ModuleFileEntry>> {
        let mut resources = Vec::with_capacity(self.resource_count as usize);
        for i in self.resource_index..self.resource_count {
            resources.push(&module.files[module.resources[i as usize] as usize]);
        }
        Ok(resources)
    }
}
