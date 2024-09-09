//! Extensions to BufReader.
//!
//! Implements `read_cstring` and `read_fixed_string` that are not present in the regular BufReader.
//! * `read_cstring:` Given a buffer, reads a character until `0x00` is encountered (null termination), returns the `String` collected. Errors when given invalid UTF-8.
//! * `read_fixed_string:` Given a buffer and size, reads characters and collects them into a `String` abd returns it. Ignores unknown UTF-8 encoding, errors out when encountering end-of-file or other IO related errors.
//! * `read_enumerable:` Reads a type that implements the "Readable" trait, which reads the type `count` times, accumulating the results into a Vec of the type. The type of which to read must be specified as a generic.
//!
//! These functions are implemented as traits in generics. Requires `<Read + Seek>` to be satisfied.
//!

use std::io::{self, BufRead, BufReader, Read, Seek};

/// "Readable" trait that makes sure a "read" method is declared.
pub trait Readable {
    fn read<R: BufRead + BufReaderExt + Seek>(&mut self, reader: &mut R) -> anyhow::Result<()>;
}

/// Extension trait for BufReader to add custom reading methods.
pub trait BufReaderExt: BufRead
where
    Self: std::io::Seek,
{
    /// Reads a UTF-8 encoded C-style string from the reader until a null terminator (0x00) is encountered.
    ///
    /// # Returns
    ///
    /// Returns an `io::Result<String>` containing the read string.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * There's an I/O error while reading from the reader.
    /// * The read bytes are not valid UTF-8.
    fn read_cstring(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        self.read_until(0, &mut buffer)?;
        if buffer.ends_with(&[0]) {
            buffer.pop(); // remove null terminator from buffer
        }
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Reads a fixed-length UTF-8 encoded string from the reader.
    ///
    /// This function reads exactly `length` bytes, ignores unknown UTF-8 encodings,
    /// and trims any null bytes found at the end of the string.
    ///
    /// # Arguments
    ///
    /// * `length` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// Returns an `io::Result<String>` containing the read string.
    ///
    /// # Errors
    ///
    /// Returns `Ok(())` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read_fixed_string(&mut self, length: usize) -> io::Result<String> {
        let mut buffer = vec![0; length];
        self.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer)
            .trim_end_matches('\0')
            .to_string())
    }

    /// Reads an enumerable type accumulating the results into a vector of the same type.
    ///
    /// This function reads a type containg the trait `Readable` `count` times, requiring the
    /// `Readable` the passed type to implement an iterator.
    ///
    /// # Arguments
    ///
    /// * `count` - The amount of times to read the specified type.
    ///
    /// # Returns
    ///
    /// Returns an `io::Result<Vec<T>>` containing the accumulated results.
    ///
    /// # Errors
    ///
    /// Returns `Ok(Vec<T>)` if the read operation is successful, or an `Err` containing
    /// the I/O error if any reading operation fails.
    fn read_enumerable<T: Default + Readable>(&mut self, count: usize) -> io::Result<Vec<T>>
    where
        Self: std::marker::Sized,
        std::vec::Vec<T>: std::iter::FromIterator<T>,
    {
        let enumerables = (0..count)
            .map(|_| {
                let mut enumerable = T::default();
                enumerable.read(self).unwrap();
                enumerable
            })
            .collect();
        Ok(enumerables)
    }
}

impl<R: Read + Seek> BufReaderExt for BufReader<R> {}
