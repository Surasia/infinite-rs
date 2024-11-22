//! Module file entry containing metadata relating to tags and functions to read them.

use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
};

use super::{block::ModuleBlockEntry, kraken::decompress};
use crate::common::errors::{ModuleError, TagError};
use crate::common::extensions::Enumerable;
use crate::tag::datablock::TagDataBlock;
use crate::tag::structure::{TagStruct, TagStructType};
use crate::{common::extensions::BufReaderExt, tag::loader::TagFile};
use crate::{Error, Result};

/// Trait for defining tag structures.
///
/// This trait is meant to be used with its derive macro, available in the `derive` feature.
/// It allows the [`read_metadata<T>`](`ModuleFileEntry::read_metadata`) function to be called on a [`ModuleFileEntry`] to read the tag data.
///
/// Each struct that implements this trait should have the following attributes:
/// - `#[data(size())]` - The size of the tag structure in bytes.
///
/// For each of its fields, the following attributes are required:
/// - `#[data(offset())]` - The offset in bytes from the start of the tag structure.
///
/// Any padding between fields should be accounted for in the offset.
///
/// # Examples
///
/// ```rust
/// use infinite_rs::module::file::ModuleFileEntry;
/// use infinite_rs_derive::TagStructure;
/// use infinite_rs::tag::types::common_types::AnyTag;
/// use infinite_rs::module::file::TagStructure;
///
/// #[derive(Default, TagStructure)]
/// #[data(size(0x30))]
/// struct MaterialTag {
///    #[data(offset(0x00))]
///    any_tag: AnyTag,
/// }
///
/// fn load_tag() {
///    let mut file_entry = ModuleFileEntry::default(); // In actual module, use reference to file.
///    let mut material = MaterialTag::default();
///    file_entry.read_metadata(&mut material).unwrap();
///
///    assert_eq!(material.size(), 0x30);
///    assert_eq!(material.offsets().get("any_tag"), Some(&0x00));
/// }
pub trait TagStructure {
    /// Returns the size of the tag structure in bytes.
    /// Determined by the [data(size())] attribute.
    fn size(&mut self) -> u64;
    /// Function that calls all [`read`](`crate::common::extensions::Enumerable::read`) functions for each field in the tag structure.
    fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()>;
    /// Returns a map of field names to their offsets in the tag structure.
    fn offsets(&self) -> HashMap<&'static str, u64>;
    /// Function that loads all field blocks for the tag structure, if any.
    fn load_field_blocks<R: BufReaderExt>(
        &mut self,
        source_index: i32,
        adjusted_base: u64,
        reader: &mut R,
        structs: &[TagStruct],
        blocks: &[TagDataBlock],
    ) -> Result<()>;
}

bitflags! {
    #[derive(Debug, Default)]
    /// Flags for the last 2 bytes of the data offset.
    pub(super) struct DataOffsetType : u16  {
        /// No additional HD1 module is required.
        const USE_SELF = 0;
        /// Additional HD1 module is required.
        const USE_HD1 = 0b0000_0001;
        /// Unknown (HD2)?
        const USE_HD2 = 0b0000_0010;
    }
}

bitflags! {
    #[derive(Debug, Default)]
    /// Flags that determine how a tag should be read.
    pub struct FileEntryFlags : u8  {
        /// If tag is compressed or not.
        const COMPRESSED = 0b0000_0001;
        /// Indicates that tag is made up of "tag blocks" which need to be joined to assemble the
        /// entire file entry.
        const HAS_BLOCKS = 0b0000_0010;
        /// "Raw tag" that does not contain a tag header.
        const RAW_FILE = 0b0000_0100;
    }
}

#[derive(Default, Debug)]
/// Module file entry structure containing metadata relating to file and required buffer sizes and offsets for the decompressor, as well as global tag ID, resource references and class.
pub struct ModuleFileEntry {
    /// Unknown, some sort of size?
    unknown: u8,
    /// Determine how the file should be read.
    pub flags: FileEntryFlags,
    /// Number of blocks that make up the file.
    block_count: u16,
    /// Index of the first block in the module.
    block_index: i32,
    /// Index of the first resource in the module's resource list.
    pub(super) resource_index: i32,
    /// 4 byte-long string for tag group, stored as big endian. This determines how the rest of the tag is read.
    /// Example:
    /// * `bitm`: Bitmap
    /// * `mat `: Material
    pub tag_group: String,
    /// Offset of compressed/uncompressed data in from the start of compressed data in the module.
    data_offset: u64,
    /// Where the offset is located. 1 if in HD1.
    pub(super) data_offset_flags: DataOffsetType,
    /// Size in bytes of compressed buffer in module.
    total_compressed_size: u32,
    /// Size in bytes of buffer to decompress into.
    total_uncompressed_size: u32,
    /// `MurmurHash3_x86_64` 32 bit hash of tag path.
    /// Referred to in-memory as "global tag id"
    /// Is set to -1 if file is resource.
    pub tag_id: i32,
    /// Size in bytes of header in decompressed buffer.
    pub(super) uncompressed_header_size: u32,
    /// Size in bytes of actual tag data in decompressed buffer.
    uncompressed_tag_data_size: u32,
    /// Size in bytes of resource data in decompressed buffer.
    uncompressed_resource_data_size: u32,
    /// Size in bytes of "external" resource data in decompressed buffer. (for instance, havok data or bitmaps)
    uncompressed_actual_resource_size: u32,
    /// Power of 2 to align the header buffer to (ex w. 4 = align to a multiple of 16 bytes).
    header_alignment: u8,
    /// Power of 2 to align the tag data buffer to.
    tag_data_alignment: u8,
    /// Power of 2 to align the resource data buffer to.
    resource_data_alignment: u8,
    /// Power of 2 to align the actual resource data buffer to.
    actual_resource_data_alignment: u8,
    /// Offset where the name of the file is located in the string table.
    /// This is no longer valid as of module version 52.
    name_offset: u32,
    /// Used with resources to point back to the parent file. -1 = none
    parent_index: i32,
    /// `Murmur3_x64_128` hash of (what appears to be) the original file that this file was built from.
    /// This is not always the same thing as the file stored in the module.
    /// Only verified if the `HasBlocks` flag is not set.
    asset_hash: i128,
    /// Number of resources owned by the file.
    pub(super) resource_count: i32,
    /// Data stream containing a buffer of bytes to read/seek.
    pub data_stream: Option<BufReader<Cursor<Vec<u8>>>>,
    /// The actual tag file read from the contents (including header), only valid if file is not a resource.
    pub tag_info: Option<TagFile>,
    /// Indicates if file is cached (has data stream) or not.
    is_loaded: bool,
}

impl Enumerable for ModuleFileEntry {
    fn read<R: BufReaderExt>(&mut self, reader: &mut R) -> Result<()> {
        self.unknown = reader.read_u8()?;
        self.flags = FileEntryFlags::from_bits_truncate(reader.read_u8()?);
        self.block_count = reader.read_u16::<LE>()?;
        self.block_index = reader.read_i32::<LE>()?;
        self.resource_index = reader.read_i32::<LE>()?;
        self.tag_group = reader.read_fixed_string(4)?.chars().rev().collect(); // Reverse string
        self.data_offset = reader.read_u64::<LE>()? & 0x0000_FFFF_FFFF_FFFF; // Mask first 6 bytes
        self.data_offset_flags =
            DataOffsetType::from_bits_truncate((self.data_offset >> 48) as u16); // Read last 2 bytes
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
        reader.seek_relative(4)?; // Skip some padding
        Ok(())
    }
}

impl ModuleFileEntry {
    /// Reads and loads tag data from a file.
    ///
    /// # Arguments
    ///
    /// * `reader` -  A mutable reference to a [`BufReader<File>`] from which to read the data.
    /// * `data_offset` - Starting offset in bytes of the data in the file.
    /// * `blocks` - Metadata for data blocks.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an [`Error`] containing
    /// the I/O error if any reading operation fails.
    pub(super) fn read_tag(
        &mut self,
        reader: &mut BufReader<File>,
        data_offset: u64,
        blocks: &[ModuleBlockEntry],
    ) -> Result<()> {
        if self.is_loaded {
            return Ok(());
        }
        let file_offset = data_offset + self.data_offset;
        let mut data = vec![0u8; self.total_uncompressed_size as usize];

        // Set position to start as we are already adding the file offset to it.
        reader.rewind()?;

        if self.block_count != 0 {
            self.read_multiple_blocks(reader, blocks, file_offset, &mut data)?;
        } else {
            read_single_block(reader, self, file_offset, &mut data)?;
        }

        let data_stream = BufReader::new(Cursor::new(data));
        self.data_stream = Some(data_stream);

        if self.tag_id != -1 {
            let mut tagfile = TagFile::default();
            if let Some(ref mut stream) = self.data_stream {
                tagfile.read(stream)?;
            }
            self.tag_info = Some(tagfile);
        }

        self.is_loaded = true;
        Ok(())
    }

    /// Reads multiple blocks of data from the file.
    ///
    /// This function reads multiple blocks of data, which can be either compressed or uncompressed,
    /// from the file and stores them in the provided data buffer.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a [`BufReader<File>`] from which to read the data.
    /// * `blocks` - A slice of [`ModuleBlockEntry`] containing metadata about each block.
    /// * `file_offset` - The offset in the file where the data blocks start.
    /// * `data` - A mutable slice where the (decompressed) data will be stored.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Error` containing
    /// the I/O error if any reading operation fails.
    #[allow(clippy::cast_sign_loss)]
    fn read_multiple_blocks(
        &self,
        reader: &mut BufReader<File>,
        blocks: &[ModuleBlockEntry],
        file_offset: u64,
        data: &mut [u8],
    ) -> Result<()> {
        if self.block_index < 0 {
            return Err(Error::ModuleError(ModuleError::NegativeBlockIndex(
                self.block_index,
            )));
        }
        let first_block_index = self.block_index as usize;
        reader.seek(SeekFrom::Start(file_offset))?;

        for block in &blocks[first_block_index..(first_block_index + self.block_count as usize)] {
            if block.is_compressed {
                read_compressed_block(reader, block, data)?;
            } else {
                read_uncompressed_block(reader, block, data)?;
            }
        }
        Ok(())
    }

    /// Reads a specified structure implementing [`TagStructure`] from the tag data.
    ///
    /// This function exhausts the inner [`data_stream`](`self.data_stream`) buffer to read the contents of the specified
    /// struct. It first looks for the main struct definition of the file, then gets the referenced
    /// data block and creates a reader for it. The initial contents of the struct are read, and
    /// field block definitions are loaded recursively.
    ///
    ///
    /// # Arguments
    ///
    /// * `struct_type` - A mutable reference to the struct implementing [`TagStructure`] to read the data into.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an [`Error`] containing
    /// the I/O error if any reading operation fails.
    pub fn read_metadata<T: Default + TagStructure>(&mut self, struct_type: &mut T) -> Result<T> {
        let mut full_tag = Vec::with_capacity(
            self.total_uncompressed_size as usize - self.uncompressed_header_size as usize,
        );
        self.data_stream
            .as_mut()
            .ok_or(Error::TagError(TagError::NotLoaded))?
            .read_to_end(&mut full_tag)?;

        let tag_info = self
            .tag_info
            .as_ref()
            .ok_or(Error::TagError(TagError::NoTagInfo))?;

        let main_struct = tag_info
            .struct_definitions
            .iter()
            .find(|s| s.struct_type == TagStructType::MainStruct)
            .ok_or(Error::TagError(TagError::MainStructNotFound))?;

        #[allow(clippy::cast_sign_loss)]
        let main_block: &TagDataBlock =
            &tag_info.datablock_definitions[main_struct.target_index as usize];
        let full_tag_buffer = &full_tag[usize::try_from(main_block.offset)?..];
        let mut full_tag_reader = BufReader::new(Cursor::new(full_tag_buffer));

        struct_type.read(&mut full_tag_reader)?;
        struct_type.load_field_blocks(
            main_struct.target_index,
            0,
            &mut full_tag_reader,
            &tag_info.struct_definitions[..],
            &tag_info.datablock_definitions[..],
        )?;

        Ok(T::default())
    }
}

/// Reads an uncompressed block of data from the file.
///
/// This function reads an uncompressed block directly from the file and copies it
/// into the appropriate section of the output buffer.
///
/// # Arguments
///
/// * `reader` - A mutable reference to a [`BufReader<File>`] from which to read the data.
/// * `block` - A reference to the [`ModuleBlockEntry`] containing metadata about the block.
/// * `data` - A mutable slice where the uncompressed data will be stored.
///
/// # Returns
///
/// Returns `Ok(())` if the read operation is successful, or an [`Error`] containing
/// the I/O error if any reading operation fails.
fn read_uncompressed_block(
    reader: &mut BufReader<File>,
    block: &ModuleBlockEntry,
    data: &mut [u8],
) -> Result<()> {
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
/// * `reader` - A mutable reference to a [`BufReader<File>`] from which to read the data.
/// * `block` - A reference to the [`ModuleBlockEntry`] containing metadata about the block.
/// * `data` - A mutable slice where the decompressed data will be stored.
///
/// # Returns
///
/// Returns `Ok(())` if the read operation is successful, or an [`Error`] containing
/// the I/O error if any reading operation fails.
fn read_compressed_block(
    reader: &mut BufReader<File>,
    block: &ModuleBlockEntry,
    data: &mut [u8],
) -> Result<()> {
    let mut compressed_data = vec![0u8; block.compressed_size as usize];
    reader.read_exact(&mut compressed_data)?;
    let mut decompressed_data = vec![0u8; block.decompressed_size as usize];
    decompress(
        &compressed_data,
        &mut decompressed_data,
        block.decompressed_size as usize,
    )?;
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
/// * `reader` - A mutable reference to a [`BufReader<File>`] from which to read the data.
/// * `file_entry` - A reference to the [`ModuleFileEntry`] containing metadata about the file.
/// * `file_offset` - The offset in the file where the data block starts.
/// * `data` - A mutable reference to the [`Vec<u8>`] where the (decompressed) data will be stored.
///
/// # Returns
///
/// Returns `Ok(())` if the read operation is successful, or an [`Error`]    containing
/// the I/O error if any reading operation fails.
fn read_single_block(
    reader: &mut BufReader<File>,
    file_entry: &ModuleFileEntry,
    file_offset: u64,
    data: &mut Vec<u8>,
) -> Result<()> {
    reader.seek(SeekFrom::Start(file_offset))?;
    let compressed_size = file_entry.total_compressed_size as usize;
    let mut block = vec![0u8; compressed_size];
    reader.read_exact(&mut block)?;

    if compressed_size == file_entry.total_uncompressed_size as usize {
        data.copy_from_slice(&block);
    } else {
        decompress(&block, data, file_entry.total_uncompressed_size as usize)?;
    }
    Ok(())
}
