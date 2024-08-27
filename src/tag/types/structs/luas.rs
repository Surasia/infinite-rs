//! LuaScriptTagDefinition definition dumped from the game.
//!
//! Source: <https://github.com/Codename-Atriox/TagStructs/blob/main/Structs/luas.xml>

use crate::{
    common::extensions::BufReaderExt,
    tag::types::common_types::{
        AnyTag, FieldBlock, FieldData, FieldLongString, FieldReference, FieldStringId,
    },
};
use std::io::{BufRead, Seek};

#[derive(Default, Debug)]
/// This type (luas) is a container for a HavokScript 5.1 bytecode file.
/// It is mainly used for UI scripts.
pub struct LuaScriptTagDefinition {
    /// VTable space, global tag id and local handle.
    pub any_tag: AnyTag,
    /// String id of the file name, used in external references.
    pub lua_file_name: FieldStringId,
    /// "Data" block which dictates how large the lua file is.
    lua_file_data: FieldData,
    /// 256 byte long string of the source file that this file was compiled from.
    pub lua_file_name_string: FieldLongString,
    /// Buffer that stores the actual lua data.
    pub lua_file: Vec<u8>,
    /// Dictates the size of the "referenced tags" block.
    referenced_tags_block: FieldBlock,
    /// List of tags referenced by the script.
    pub referenced_tags: Vec<FieldReference>,
}

impl LuaScriptTagDefinition {
    /// Allocate new LuaScriptTagDefinition and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the luas structure type from the given readers implementing BufRead, BufReaderExt and Seek.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `BufRead + BufReaderExt + Seek` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read<R: BufRead + BufReaderExt + Seek>(
        &mut self,
        reader: &mut R,
    ) -> std::io::Result<()> {
        self.any_tag.read(reader)?;
        self.lua_file_name.read(reader)?;
        self.lua_file_data.read(reader)?;
        self.lua_file_name_string.read(reader)?;
        self.referenced_tags_block.read(reader)?;
        let mut lua_file_buffer = vec![0u8; self.lua_file_data.size as usize];
        reader.read_exact(&mut lua_file_buffer)?;
        self.lua_file = lua_file_buffer;

        // Align to 4 bytes after reading the buffer
        let alignment = (4 - (reader.stream_position()? % 4)) % 4;
        reader.seek(std::io::SeekFrom::Current(alignment as i64))?;

        self.referenced_tags = (0..self.referenced_tags_block.size)
            .map(|_| {
                let mut reference = FieldReference::default();
                reference.read(reader).unwrap();
                reference
            })
            .collect();

        Ok(())
    }
}
