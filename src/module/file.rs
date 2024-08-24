//! Module file entry containing metadata relating to tags and functions to read them.

use super::{block::ModuleBlockEntry, kraken::decompress};
use crate::{common::extensions::BufReaderExt, tag::loader::TagFile};
use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read, Seek, SeekFrom},
    path::Path,
};

bitflags! {
    #[derive(Debug, Default)]
    /// Flags for the last 2 bytes of the data offset.
    pub struct DataOffsetType : u16  {
        const USE_SELF = 0b00000000;
        const USE_HD1 = 0b00000001;
        const USE_HD2 = 0b00000010;
    }
}

#[derive(Default, Debug)]
/// Module file entry structure containing metadata relating to file and required buffer sizes and offsets for the decompressor, as well as global tag ID, resource references and class.
pub struct ModuleFileEntry {
    pub resource_count: u32,
    pub parent_index: i32,
    pub unknown: u16,
    pub block_count: u16,
    pub block_index: u32,
    pub resource_index: u32,
    pub class_id: String,
    pub data_offset: u64,
    pub data_offset_flags: DataOffsetType,
    pub total_compressed_size: u32,
    pub total_uncompressed_size: u32,
    pub global_tag_id: i32,
    pub uncompressed_header_size: u32,
    pub uncompressed_tag_data_size: u32,
    pub uncompressed_resource_data_size: u32,
    pub uncompressed_actual_resource_size: u32,
    pub resource_block_count: u32,
    pub name_offset: u32,
    pub parent_resource: i32,
    pub asset_checksum: u64,
    pub asset_id: u64,
    pub data_stream: Cursor<Vec<u8>>,
    pub metadata: TagFile,
    pub is_loaded: bool,
}

impl ModuleFileEntry {
    /// Reads the metadata of a module file entry from the given reader.
    ///
    /// This function populates the fields of the `ModuleFileEntry` struct by reading
    /// various data types from the provided `BufReader<File>`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BufReader<File>` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        self.resource_count = reader.read_u32::<LE>()?;
        self.parent_index = reader.read_i32::<LE>()?;
        self.unknown = reader.read_u16::<LE>()?;
        self.block_count = reader.read_u16::<LE>()?;
        self.block_index = reader.read_u32::<LE>()?;
        self.resource_index = reader.read_u32::<LE>()?;
        self.class_id = reader.read_fixed_string(4)?.chars().rev().collect(); // Reverse string
        self.data_offset = reader.read_u64::<LE>()? & 0x0000FFFFFFFFFFFF;
        self.data_offset_flags =
            DataOffsetType::from_bits_truncate((self.data_offset >> 48) as u16);
        self.total_compressed_size = reader.read_u32::<LE>()?;
        self.total_uncompressed_size = reader.read_u32::<LE>()?;
        self.global_tag_id = reader.read_i32::<LE>()?;
        self.uncompressed_header_size = reader.read_u32::<LE>()?;
        self.uncompressed_tag_data_size = reader.read_u32::<LE>()?;
        self.uncompressed_resource_data_size = reader.read_u32::<LE>()?;
        self.uncompressed_actual_resource_size = reader.read_u32::<LE>()?;
        self.resource_block_count = reader.read_u32::<LE>()?;
        self.name_offset = reader.read_u32::<LE>()?;
        self.parent_resource = reader.read_i32::<LE>()?;
        self.asset_checksum = reader.read_u64::<LE>()?;
        self.asset_id = reader.read_u64::<LE>()?;
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
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read_tag(
        &mut self,
        file_path: &String,
        data_offset: u64,
        blocks: &[ModuleBlockEntry],
    ) -> std::io::Result<()> {
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

            let data_stream = std::io::Cursor::new(data);
            self.data_stream.clone_from(&data_stream);

            let mut buf_reader = BufReader::new(data_stream);
            self.metadata.read(&mut buf_reader)?;
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
}
