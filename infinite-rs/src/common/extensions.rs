//! Extensions to `BufReader`.
//!
//! Implements `read_fixed_string` and `read_enumerable` that are not present in the regular `BufReader`.
//! * `read_fixed_string:` Given a buffer and size, reads characters and collects them into a `String` and returns it.
//! * `read_enumerable:` Reads a type that implements the `Readable` trait, which reads the type `count` times, accumulating the results into a `Vec` of the type. The type to read must be specified as a generic.
//!
//! These functions are implemented as traits in generics. Requires `<Read + Seek>` to be satisfied.
//!

use std::io::{BufRead, BufReader, Read, Seek};

use crate::Result;

/// `Readable` trait that ensures a `read` method is declared.
pub trait Readable {
    /// Reads data from the given reader and processes it.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to an object that implements `BufRead`, `BufReaderExt`, and `Seek`.
    ///
    /// # Returns
    ///
    /// Returns a `Result<(), Error>` indicating the success or failure of the read operation.
    ///
    /// # Errors
    ///
    /// Returns an `Err` containing the error if the read operation fails.
    fn read<R>(&mut self, reader: &mut R) -> Result<()>
    where
        R: BufRead + BufReaderExt + Seek;
}

/// Extension trait for `BufReader` to add custom reading methods.
pub trait BufReaderExt: BufRead
where
    Self: Seek,
{
    /// Reads a fixed-length UTF-8 encoded string from the reader.
    ///
    /// This function reads exactly `length` bytes into a UTF-8 string and trims any null bytes found at the end of the string.
    ///
    /// # Arguments
    ///
    /// * `length` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// Returns a `Result<String, Error>` containing the read string.
    ///
    /// # Errors
    ///
    /// Returns `Ok(String)` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use std::io::BufReader;
    /// use infinite_rs::common::extensions::BufReaderExt;
    ///
    /// let data = b"I love cats!";
    /// let mut reader = BufReader::new(Cursor::new(data));
    /// let string = reader.read_fixed_string(data.len()).unwrap();
    /// assert_eq!(string, "I love cats!");
    /// ```
    fn read_fixed_string(&mut self, length: usize) -> Result<String> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;

        if buffer == [255, 255, 255, 255] {
            return Ok(String::new()); // Return empty string if all bytes are 0xFF
        }

        let string = String::from_utf8(buffer)?;

        Ok(string)
    }

    /// Reads an enumerable type, accumulating the results into a vector of the same type.
    ///
    /// This function reads a type implementing the `Readable` trait `count` times.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of times to read the specified type.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Vec<T>, Error>` containing the accumulated results.
    ///
    /// # Errors
    ///
    /// Returns `Ok(Vec<T>)` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Cursor, BufReader, BufRead, Seek};
    /// use infinite_rs::common::extensions::{BufReaderExt, Readable};
    /// use infinite_rs::common::errors::Error;
    /// use byteorder::{ReadBytesExt, LE};
    ///
    /// #[derive(Default)]
    /// struct TestType {
    ///     value: u32,
    /// }
    ///
    /// impl Readable for TestType {
    ///     fn read<R>(&mut self, reader: &mut R) -> Result<(), Error>
    ///     where
    ///         R: BufRead + BufReaderExt + Seek,
    ///     {
    ///         self.value = reader.read_u32::<LE>()?;
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let data = b"\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00";
    /// let mut reader = BufReader::new(Cursor::new(data));
    /// let enumerables = reader.read_enumerable::<TestType>(3).unwrap();
    /// assert_eq!(enumerables.len(), 3);
    /// assert_eq!(enumerables[0].value, 1);
    /// assert_eq!(enumerables[1].value, 2);
    /// assert_eq!(enumerables[2].value, 3);
    /// ```
    fn read_enumerable<T: Default + Readable>(&mut self, count: u64) -> Result<Vec<T>>
    where
        Self: Sized,
        Vec<T>: FromIterator<T>,
    {
        let enumerables = (0..count)
            .map(|_| -> Result<T> {
                let mut enumerable = T::default();
                enumerable.read(self)?;
                Ok(enumerable)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(enumerables)
    }
}

impl<R: Read + Seek> BufReaderExt for BufReader<R> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    /// Some strings such as the `tag_group` of the first file entry in the modules are empty (0xFFFFFFFF), which should be handled to return an empty string.
    fn test_read_fixed_string_empty() {
        let data = [255, 255, 255, 255];
        let mut reader = BufReader::new(Cursor::new(&data));
        let string = reader.read_fixed_string(data.len()).unwrap();
        assert_eq!(string, "");
    }
}
