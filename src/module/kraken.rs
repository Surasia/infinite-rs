//! Kraken decompressor wrapper.

#[link(name = "kraken_static")]
extern "C" {
    // EXPORT int Kraken_Decompress(const byte *src, size_t src_len, byte *dst, size_t dst_len)
    fn Kraken_Decompress(
        buffer: *const u8,
        bufferSize: i64,
        outputBuffer: *mut u8,
        outputBufferSize: i64,
    ) -> i32;
}

/// UNSAFE: Decompresses a Kraken-compressed buffer.
///
/// # Arguments
///
/// * `compressed_buffer` - A vector containing the compressed data.
/// * `output_buffer` - A mutable reference to a vector where the decompressed data will be stored.
/// * `size` - The expected size of the decompressed data.
///
/// # Returns
///
/// An `i32` representing the result of the decompression operation.
pub fn decompress(compressed_buffer: Vec<u8>, output_buffer: &mut Vec<u8>, size: usize) -> i32 {
    let mut buffer = vec![0; size * 2];
    let result;

    unsafe {
        result = Kraken_Decompress(
            compressed_buffer.as_ptr(),
            compressed_buffer.len() as i64,
            buffer.as_mut_ptr(),
            size as i64,
        );

        buffer.resize(result as usize, 0);
        *output_buffer = buffer;
    }

    result
}
