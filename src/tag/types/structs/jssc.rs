//! JsonSourceFileTag definition dumped from the game.
//!
//! Source: <https://github.com/Codename-Atriox/TagStructs/blob/main/Structs/jssc.xml>

use crate::{
    common::extensions::BufReaderExt,
    tag::types::common_types::{AnyTag, FieldData, FieldLongString, FieldReference, FieldStringId},
};
use std::io::{Read, Seek};

#[derive(Default, Debug)]
/// This type (jssc) contains some metadata regarding the file stored in it and the contents of it, which is a JSONC (JSON with comments) file.
///
/// This file is only used in Forge node definitions.
pub struct JsonSourceFileTagDefinition {
    /// VTable space, global tag id and local handle.
    pub any_tag: AnyTag,
    /// Reference to the tag this tag originates from.
    /// Invalid in release builds.
    pub schema_file_reference: FieldReference,
    /// String ID of the file name, used for referencing it.
    pub schema_file_name: FieldStringId,
    /// String of the path that this file originates from.
    pub schema_file_name_string: FieldLongString,
    /// Metadata regarding where the actual JSON is stored in the file.
    pub schema_file_data: FieldData,
    /// JSONC String, the actual data.
    pub data: String,
}

impl JsonSourceFileTagDefinition {
    /// Allocate new JsonSourceFileTagDefinition and set it to default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// Reads the jssc structure type from the given readers implementing Read, BufReaderExt and Seek.
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a reader that implements `Read + BufReaderExt + Seek` from which to read the data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    pub fn read<R: Read + BufReaderExt + Seek>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.any_tag.read(reader)?;
        self.schema_file_reference.read(reader)?;
        self.schema_file_name.read(reader)?;
        self.schema_file_name_string.read(reader)?;
        self.schema_file_data.read(reader)?;
        // Read one less byte: gets rid of null terminator, forming proper JSONC.
        self.data = reader.read_fixed_string(self.schema_file_data.size as usize - 1)?;
        Ok(())
    }
}
