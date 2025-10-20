/// This module exposes low-level methods for reading/writing from byte streams or buffers.
use std::io::{self};
use thiserror::Error;

/// A custom error type for varint encoding and decoding operations.
///
/// This enum provides specific error variants for different failure conditions,
/// making the error handling clearer and more robust.
#[derive(Error, Debug)]
pub enum VarintError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Varint is too long")]
    VarintTooLong,
    #[error("Unterminated varint")]
    UnterminatedVarint,
}

/// A type alias for a `Result` that uses our custom `VarintError`.
pub type VarintResult<T> = Result<T, VarintError>;

/// Reads a 4-byte unsigned integer from a buffer, advancing the buffer's position by 4 bytes.
///
/// This function is analogous to `ByteBuffer.getInt() & 0xffffffffL` in Java,
/// which reads a 32-bit signed integer and then masks it to be treated as an unsigned 32-bit
/// value.
/// In Rust, this is achieved more directly by reading bytes and converting them into a `u32`
/// (unsigned 32-bit integer).
///
/// # Arguments
///
/// * `buffer` - A mutable reference to a type that implements the `std::io::Read` trait.
///
/// # Returns
///
/// The unsigned 32-bit integer read from the buffer as a `u32`.
///
/// # Panics
///
/// This function will panic if it fails to read exactly 4 bytes from the buffer.
pub fn read_unsigned_int(buffer: &mut dyn io::Read) -> u32 {
    let mut bytes = [0; 4];
    buffer.read_exact(&mut bytes).unwrap();
    u32::from_be_bytes(bytes)
}

/// Reads a 4-byte unsigned integer from a specific index in a byte slice.
///
/// This function does not modify the position of the underlying buffer. It is analogous
/// to `ByteBuffer.getInt(index)` in Java, which provides indexed, non-sequential access.
///
/// # Arguments
///
/// * `buffer` - A reference to the byte slice to read from.
/// * `index` - The starting byte index from which to read the 4-byte integer.
///
/// # Returns
///
/// The unsigned 32-bit integer read from the buffer as a `u32`.
///
/// # Panics
///
/// This function will panic if the provided `index` is out of bounds or if there are
/// fewer than 4 bytes available from the given index.
pub fn read_unsigned_int_at(buffer: &[u8], index: usize) -> u32 {
    let bytes: [u8; 4] = buffer[index..index + 4].try_into().unwrap();
    u32::from_be_bytes(bytes)
}

/// Reads a 4-byte signed integer from a specific index in a byte slice in big-endian byte
/// order.
///
/// This function is the Rust equivalent of the provided Java `readIntBE` function.
/// It uses the `i32::from_be_bytes` method, which is a safe and idiomatic way to
/// convert a 4-byte array into a signed 32-bit integer.
///
/// # Arguments
///
/// * `buffer` - A reference to the byte slice to read from.
/// * `offset` - The starting byte offset from which to read the 4-byte integer.
///
/// # Returns
///
/// The signed 32-bit integer read from the buffer as an `i32`.
///
/// # Panics
///
/// This function will panic if the provided `offset` is out of bounds or if there
/// are fewer than 4 bytes available from the given offset.
pub fn read_int_be(buffer: &[u8], offset: usize) -> i32 {
    let bytes: [u8; 4] = buffer[offset..offset + 4].try_into().unwrap();
    i32::from_be_bytes(bytes)
}

/// Writes a 4-byte unsigned integer to a specific index in a mutable byte slice.
///
/// This function is the Rust equivalent of the provided Java `writeUnsignedInt` function.
/// It takes a `u32` value, converts it to a big-endian byte array, and writes it to the
/// specified location in the buffer without modifying its position. Overflow is handled
/// by the `u32` type's natural wrapping behavior if a larger value is provided.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the byte slice to write to.
/// * `index` - The starting byte index from which to begin writing the 4-byte integer.
/// * `value` - The unsigned 32-bit integer to write.
///
/// # Panics
///
/// This function will panic if the provided `index` is out of bounds or if there
/// are fewer than 4 bytes available from the given index to write to.
pub fn write_unsigned_int_at(buffer: &mut [u8], index: usize, value: u32) {
    buffer[index..index + 4].copy_from_slice(&value.to_be_bytes());
}

/// Writes a 32-bit unsigned integer to a buffer in little-endian format.
///
/// This function is the Rust equivalent of the provided Java `writeUnsignedIntLE`.
/// It takes a `u32` value, converts it to a 4-byte array in little-endian byte order,
/// and writes the bytes to the given output stream.
///
/// # Arguments
///
/// * `out` - A mutable reference to a type that implements the `std::io::Write` trait.
/// * `value` - The unsigned 32-bit integer to write.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `io::Result` containing the error on failure.
pub fn write_unsigned_int(buffer: &mut dyn io::Write, value: u32) {
    buffer.write_all(&value.to_be_bytes()).unwrap();
}

/// Writes an unsigned 32-bit integer in little-endian format to the writer.
///
/// # Arguments
///
/// * `writer` - The writer to write to.
/// * `value` - The value to write.
pub fn write_unsigned_int_le(writer: &mut impl io::Write, value: u32) -> io::Result<()> {
    writer.write_all(&value.to_le_bytes())
}

/// Writes a 32-bit unsigned integer to a byte slice at a given offset in little-endian format.
///
/// This function is the Rust equivalent of the provided Java `writeUnsignedIntLE`. It takes
/// a `u32` value, converts it to a 4-byte array in little-endian byte order, and writes
/// those bytes to the specified location in the given mutable slice.
///
/// # Arguments
///
/// * `buffer` - A mutable slice of bytes to write to.
/// * `offset` - The starting byte index from which to begin writing.
/// * `value` - The unsigned 32-bit integer to write.
///
/// # Panics
///
/// This function will panic if the provided `offset` is out of bounds or if there are
/// fewer than 4 bytes available from the given offset to write to.
pub fn write_unsigned_int_le_at(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// Writes a 4-byte signed integer to a specific index in a mutable byte slice in big-endian
/// byte order.
///
/// This function is the Rust equivalent of `ByteBuffer.putInt(index, value)`. It converts a
/// signed 32-bit integer to a big-endian byte array and writes it to the specified position.
///
/// # Arguments
///
/// * `buffer` - A mutable reference to the byte slice to write to.
/// * `index` - The starting byte index from which to begin writing the 4-byte integer.
/// * `value` - The signed 32-bit integer to write.
///
/// # Panics
///
/// This function will panic if the provided `index` is out of bounds or if there
/// are fewer than 4 bytes available from the given index to write to.
#[cfg(test)]
pub fn write_int_be(buffer: &mut [u8], index: usize, value: i32) {
    buffer[index..index + 4].copy_from_slice(&value.to_be_bytes());
}

/// Reads an unsigned 32-bit integer in variable-length format from a reader.
///
/// This implementation is based on the decoding of varint from Google Protocol Buffers.
///
/// # Arguments
///
/// * `reader` - A mutable reference to an object that implements the `Read` trait.
///
/// # Errors
///
/// Returns a `VarintError` if the varint is malformed or if there's an I/O error.
///
/// # Examples
///
/// ```rust
/// use rafka_clients::common::utils::byte_utils::read_unsigned_varint;
/// use std::io::Cursor;
///
/// let encoded_value: [u8; 2] = [0b10101100, 0b00000010]; // Represents the number 300
/// let mut cursor = Cursor::new(encoded_value);
/// let value = read_unsigned_varint(&mut cursor).unwrap();
/// assert_eq!(value, 300);
/// ```
pub fn read_unsigned_varint<R: io::Read>(reader: &mut R) -> VarintResult<u32> {
    let mut result = 0u32;
    let mut shift = 0;

    for i in 0..5 {
        let mut buffer = [0u8; 1];
        // The ? operator will propagate I/O errors, wrapped into VarintError::Io
        reader.read_exact(&mut buffer)?;
        let byte = buffer[0];

        // This is the critical check for the 5th byte, which was missing.
        // If it's the 5th byte, its MSB must be 0.
        if i == 4 && (byte & 0x80) != 0 {
            return Err(VarintError::VarintTooLong);
        }

        // Add the lower 7 bits to the result
        result |= ((byte & 0x7f) as u32) << shift;

        // If the MSB is 0, we are done
        if (byte & 0x80) == 0 {
            return Ok(result);
        }

        shift += 7;
    }

    // This case is now only reachable if the underlying reader ends
    // unexpectedly, but read_exact should catch that. It's good practice
    // to have a fallback error for an unexpected loop termination.
    Err(VarintError::UnterminatedVarint)
}

/// Reads a signed 32-bit integer from a variable-length format using zig-zag decoding,
/// as defined by [Google Protocol Buffers](http://code.google.com/apis/protocolbuffers/docs/encoding.html).
///
/// Zig-zag encoding is a pre-processing step that transforms signed integers into unsigned
/// integers in a way that is highly compatible with Varint encoding.
/// It ensures that signed numbers with a small absolute value (like -1, 2, -5) are mapped
/// to small unsigned numbers, allowing them to be stored in a single byte, saving massive
/// amounts of space.
///
/// It first reads an unsigned varint and then decodes the zig-zag format.
///
/// # Type Parameters
///
/// * `R`: A type that implements the `io::Read` trait (e.g., `File`, `&[u8]`).
///
/// # Arguments
///
/// * `reader`: A mutable reference to the input source to read from.
///
/// # Returns
///
/// On success, returns `Ok(i32)` containing the decoded signed integer.
///
/// # Errors
///
/// This function will propagate any errors encountered during the underlying
/// unsigned varint read, such as `VarintError::Io` or `VarintError::VarintTooLong`.
pub fn read_varint<R: io::Read>(reader: &mut R) -> VarintResult<i32> {
    // Read the raw unsigned varint from the stream. The `?` operator will
    // propagate any errors, such as I/O issues or a varint that's too long.
    let unsigned_value = read_unsigned_varint(reader)?;

    // Perform zig-zag decoding to convert the unsigned value back to signed.
    // (n >>> 1) ^ -(n & 1) in Java/C becomes the following in Rust:
    Ok((unsigned_value >> 1) as i32 ^ (-((unsigned_value & 1) as i32)))
}

/// Reads a signed 64-bit integer from a variable-length format using zig-zag decoding,
/// as defined by [Google Protocol Buffers](http://code.google.com/apis/protocolbuffers/docs/encoding.html).
///
/// Zig-zag encoding is a pre-processing step that transforms signed integers into unsigned
/// integers in a way that is highly compatible with Varint encoding.
/// It ensures that signed numbers with a small absolute value (like -1, 2, -5) are mapped
/// to small unsigned numbers, allowing them to be stored in a single byte, saving massive
/// amounts of space.
///
/// It first reads an unsigned varint and then decodes the zig-zag format.
///
/// # Type Parameters
///
/// * `R`: A type that implements the `io::Read` trait (e.g., `File`, `&[u8]`).
///
/// # Arguments
///
/// * `reader`: A mutable reference to the input source to read from.
///
/// # Returns
///
/// On success, returns `Ok(i64)` containing the decoded signed integer.
///
/// # Errors
///
/// This function will propagate any errors encountered during the underlying
/// unsigned varint read, such as `VarintError::Io` or `VarintError::VarintTooLong`.
pub fn read_varint64<R: io::Read>(reader: &mut R) -> VarintResult<i64> {
    // Read the raw unsigned varint from the stream. The `?` operator will
    // propagate any errors, such as I/O issues or a varint that's too long.
    let unsigned_value = read_unsigned_varint64(reader)?;

    // Perform zig-zag decoding to convert the unsigned value back to signed.
    // (n >>> 1) ^ -(n & 1) in Java/C becomes the following in Rust:
    Ok((unsigned_value >> 1) as i64 ^ (-((unsigned_value & 1) as i64)))
}

/// Reads an unsigned variable-length 64-bit integer from a reader.
fn read_unsigned_varint64<R: io::Read>(reader: &mut R) -> VarintResult<u64> {
    let mut result = 0u64;
    let mut shift = 0;

    // A u64 varint can be at most 10 bytes long.
    for i in 0..10 {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;
        let byte = buffer[0];

        // On the 10th byte, the MSB must be 0. If it's set, the varint is too long.
        if i == 9 && (byte & 0x80) != 0 {
            return Err(VarintError::VarintTooLong);
        }

        // Add the lower 7 bits of the byte to the result.
        result |= ((byte & 0x7f) as u64) << shift;

        // If the most significant bit (MSB) is 0, we're done.
        if (byte & 0x80) == 0 {
            return Ok(result);
        }

        shift += 7;
    }

    // This code is unreachable if the loop runs 10 times, as the check
    // for the 10th byte will either return an error or the `read_exact` will fail.
    // It's included for logical completeness.
    Err(VarintError::UnterminatedVarint)
}

/// Writes the given unsigned 32-bit integer following the variable-length unsigned
/// encoding from Google Protocol Buffers to a writer.
///
/// This implementation is a direct translation of a Java implementation from
/// the `varint-writing-showdown` repository.
///
/// # Arguments
///
/// * `value` - The unsigned 32-bit integer to write.
/// * `writer` - A mutable reference to an object that implements the `Write` trait.
///
/// # Returns
///
/// Returns a `VarintError` if an I/O error occurs during the write operation.
///
/// # Errors
///
/// This function will return an `Err` if the underlying write operation to the
/// writer fails at any point.
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
/// use rafka_clients::common::utils::byte_utils::write_unsigned_varint;
///
/// let mut cursor = Cursor::new(Vec::new());
/// write_unsigned_varint(300, &mut cursor).unwrap();
///
/// let encoded_value = cursor.into_inner();
/// let expected_bytes: [u8; 2] = [0xAC, 0x02];
/// assert_eq!(encoded_value, expected_bytes);
/// ```
pub fn write_unsigned_varint<W: io::Write>(value: u32, writer: &mut W) -> VarintResult<()> {
    if (value & !((1 << 7) - 1)) == 0 {
        writer.write_all(&[value as u8])?;
    } else {
        writer.write_all(&[((value & 0x7F) as u8) | 0x80])?;
        if (value & !((1 << 14) - 1)) == 0 {
            writer.write_all(&[((value >> 7) & 0xFF) as u8])?;
        } else {
            writer.write_all(&[((value >> 7) & 0x7F | 0x80) as u8])?;
            if (value & !((1 << 21) - 1)) == 0 {
                writer.write_all(&[((value >> 14) & 0xFF) as u8])?;
            } else {
                writer.write_all(&[((value >> 14) & 0x7F | 0x80) as u8])?;
                if (value & !((1 << 28) - 1)) == 0 {
                    writer.write_all(&[((value >> 21) & 0xFF) as u8])?;
                } else {
                    writer.write_all(&[((value >> 21) & 0x7F | 0x80) as u8])?;
                    writer.write_all(&[((value >> 28) & 0xFF) as u8])?;
                }
            }
        }
    }
    Ok(())
}

/// Writes a signed 32-bit integer to a writer using variable-length zig-zag encoding,
/// as defined by [Google Protocol Buffers](http://code.google.com/apis/protocolbuffers/docs/encoding.html).
///
/// Zig-zag encoding maps signed integers to unsigned integers so that numbers with a
/// small absolute value (positive or negative) have a small encoded value, which
/// is more efficient for varint storage. This function first applies the encoding
/// and then calls `write_unsigned_varint` to perform the final write.
///
/// # Type Parameters
///
/// * `W`: A type that implements the `io::Write` trait (e.g., `File`, `Vec<u8>`, `TcpStream`).
///
/// # Arguments
///
/// * `value`: The `i32` value to be encoded and written.
/// * `writer`: A mutable reference to the output destination.
///
/// # Returns
///
/// Returns a `VarintError` if an I/O error occurs during the write operation.
///
/// # Errors
///
/// This function will return an `Err` if the underlying write operation to the
/// writer fails at any point.
///
/// # Example
///
/// ```
/// // Import the necessary functions
/// use std::io::Write;
/// use rafka_clients::common::utils::byte_utils::write_varint;
///
/// // Create an in-memory buffer to act as our writer.
/// let mut buffer: Vec<u8> = Vec::new();
///
/// // Write the value -1. Zig-zag encoding turns -1 into unsigned 1.
/// write_varint(-1, &mut buffer).unwrap();
///
/// // The varint encoding for 1 is a single byte: 0x01.
/// assert_eq!(buffer, vec![0x01]);
///
/// // Clear the buffer and write another value.
/// buffer.clear();
/// write_varint(1, &mut buffer).unwrap(); // Zig-zag(1) -> 2
/// assert_eq!(buffer, vec![0x02]);
///
/// buffer.clear();
/// write_varint(-2, &mut buffer).unwrap(); // Zig-zag(-2) -> 3
/// assert_eq!(buffer, vec![0x03]);
/// ```
pub fn write_varint<W: io::Write>(value: i32, writer: &mut W) -> VarintResult<()> {
    // Perform zig-zag encoding:
    // (value << 1) shifts the number left.
    // (value >> 31) creates a mask of all 1s for negative numbers (arithmetic shift)
    // or all 0s for positive numbers. XORing with this mask correctly maps the value.
    // The result is cast to u32 for the unsigned varint writer.
    let encoded = ((value << 1) ^ (value >> 31)) as u32;

    write_unsigned_varint(encoded, writer)
}

/// Encodes a u64 into a variable-length integer and writes it to a writer.
///
/// # Arguments
///
/// * `value` - The u64 value to encode.
/// * `writer` - A mutable reference to a type that implements `io::Write`,
///   where the encoded bytes will be written.
/// # Returns
///
/// Returns a `VarintError` if an I/O error occurs during the write operation.
///
/// # Errors
///
/// This function will return an `Err` if the underlying write operation to the
/// writer fails at any point.
pub fn write_unsigned_varint64<W: io::Write>(mut value: u64, writer: &mut W) -> VarintResult<()> {
    // While the value is too large to fit in the final 7 bits,
    // write continuation bytes.
    while value >= 0x80 {
        // 1. Take the lower 7 bits of the value.
        // 2. Set the most significant bit (MSB) to 1 to indicate more bytes are coming.
        let byte_to_write = (value as u8 & 0x7f) | 0x80;

        // Write the single byte to the writer.
        writer.write_all(&[byte_to_write])?;

        // Unsigned right shift the value by 7 bits to process the next chunk.
        value >>= 7;
    }

    // Write the final byte. The MSB is 0, indicating the end of the varlong.
    writer.write_all(&[value as u8])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn test_unsigned_varint_serde() {
        assert_unsigned_varint_serde(0, &[0x0]);
        assert_unsigned_varint_serde(-1i32 as u32, &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
        assert_unsigned_varint_serde(1, &[0x01]);
        assert_unsigned_varint_serde(63, &[0x3F]);
        assert_unsigned_varint_serde(-64i32 as u32, &[0xC0, 0xFF, 0xFF, 0xFF, 0x0F]);
        assert_unsigned_varint_serde(64, &[0x40]);
        assert_unsigned_varint_serde(8191, &[0xFF, 0x3F]);
        assert_unsigned_varint_serde(-8192i32 as u32, &[0x80, 0xC0, 0xFF, 0xFF, 0x0F]);
        assert_unsigned_varint_serde(8192, &[0x80, 0x40]);
        assert_unsigned_varint_serde(-8193i32 as u32, &[0xFF, 0xBF, 0xFF, 0xFF, 0x0F]);
        assert_unsigned_varint_serde(1048575, &[0xFF, 0xFF, 0x3F]);
        assert_unsigned_varint_serde(1048576, &[0x80, 0x80, 0x40]);
        assert_unsigned_varint_serde(i32::MAX as u32, &[0xFF, 0xFF, 0xFF, 0xFF, 0x07]);
        assert_unsigned_varint_serde(i32::MIN as u32, &[0x80, 0x80, 0x80, 0x80, 0x08]);
    }

    #[test]
    fn test_varint_serde() {
        assert_varint_serde(0, &[0x00]);
        assert_varint_serde(-1, &[0x01]);
        assert_varint_serde(1, &[0x02]);
        assert_varint_serde(63, &[0x7E]);
        assert_varint_serde(-64, &[0x7F]);
        assert_varint_serde(64, &[0x80, 0x01]);
        assert_varint_serde(-65, &[0x81, 0x01]);
        assert_varint_serde(8191, &[0xFE, 0x7F]);
        assert_varint_serde(-8192, &[0xFF, 0x7F]);
        assert_varint_serde(8192, &[0x80, 0x80, 0x01]);
        assert_varint_serde(-8193, &[0x81, 0x80, 0x01]);
        assert_varint_serde(1048575, &[0xFE, 0xFF, 0x7F]);
        assert_varint_serde(-1048576, &[0xFF, 0xFF, 0x7F]);
        assert_varint_serde(1048576, &[0x80, 0x80, 0x80, 0x01]);
        assert_varint_serde(-1048577, &[0x81, 0x80, 0x80, 0x01]);
        assert_varint_serde(134217727, &[0xFE, 0xFF, 0xFF, 0x7F]);
        assert_varint_serde(-134217728, &[0xFF, 0xFF, 0xFF, 0x7F]);
        assert_varint_serde(134217728, &[0x80, 0x80, 0x80, 0x80, 0x01]);
        assert_varint_serde(-134217729, &[0x81, 0x80, 0x80, 0x80, 0x01]);
        assert_varint_serde(i32::MAX, &[0xFE, 0xFF, 0xFF, 0xFF, 0x0F]);
        assert_varint_serde(i32::MIN, &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
    }

    #[test]
    fn test_read_write_unsigned_int() {
        // Create an in-memory buffer (a vector of bytes)
        let mut buffer = Vec::with_capacity(4);
        let write_value: u32 = 133444;

        // Write the value to the buffer
        write_unsigned_int(&mut buffer, write_value);

        // Create a cursor to simulate reading from the start of the buffer
        let mut cursor = Cursor::new(buffer);

        // Read the value back from the cursor
        let read_value = read_unsigned_int(&mut cursor);

        // Assert that the value read is the same as the value written
        assert_eq!(read_value, write_value);
    }

    #[test]
    fn test_read_write_int_be() {
        let values: [i32; 11] = [
            0,
            1,
            -1,
            i8::MAX as i32,
            i16::MAX as i32,
            2 * i16::MAX as i32,
            i32::MAX / 2,
            i32::MIN / 2,
            i32::MAX,
            i32::MIN,
            i32::MAX,
        ];
        let mut buffer = vec![0u8; 4 * values.len()];
        for (i, &value) in values.iter().enumerate() {
            write_int_be(&mut buffer, i * 4, value);
            assert_eq!(
                read_int_be(&buffer, i * 4),
                value,
                "Written value should match read value."
            );
        }
    }

    #[test]
    fn test_write_unsigned_int_le() {
        // Test case 1
        let value1 = 0x04030201;
        let mut buffer1 = Cursor::new(Vec::new());
        write_unsigned_int_le(&mut buffer1, value1).unwrap();
        write_unsigned_int_le(&mut buffer1, value1).unwrap();

        // Assert the written bytes are as expected
        assert_eq!(
            buffer1.into_inner(),
            vec![0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04]
        );

        // Test case 2
        let value2 = 0xf4f3f2f1;
        let mut buffer2 = Cursor::new(Vec::new());
        write_unsigned_int_le(&mut buffer2, value2).unwrap();

        // Assert the written bytes are as expected
        assert_eq!(buffer2.into_inner(), vec![0xf1, 0xf2, 0xf3, 0xf4]);
    }

    #[test]
    fn test_write_unsigned_int_le_to_array() {
        // Test with a positive value
        let value1: u32 = 0x04030201;

        let mut array1 = vec![0u8; 4];
        write_unsigned_int_le_at(&mut array1, 0, value1);
        assert_eq!(array1, vec![0x01, 0x02, 0x03, 0x04]);

        let mut array1_large = vec![0u8; 8];
        write_unsigned_int_le_at(&mut array1_large, 2, value1);
        assert_eq!(array1_large, vec![0, 0, 0x01, 0x02, 0x03, 0x04, 0, 0]);

        // Test with a negative value (treated as unsigned)
        let value2: u32 = 0xf4f3f2f1;

        let mut array2 = vec![0u8; 4];
        write_unsigned_int_le_at(&mut array2, 0, value2);
        assert_eq!(array2, vec![0xf1, 0xf2, 0xf3, 0xf4]);

        let mut array2_large = vec![0u8; 8];
        write_unsigned_int_le_at(&mut array2_large, 2, value2);
        assert_eq!(array2_large, vec![0, 0, 0xf1, 0xf2, 0xf3, 0xf4, 0, 0]);
    }

    #[test]
    fn test_correctness_read_unsigned_varint() {
        // The "simpleImplementation" from the Java test, converted to a Rust closure.
        // It takes anything that implements `io::Read` and returns a Result.
        let simple_read_impl = |reader: &mut dyn Read| -> Result<u32, &'static str> {
            let mut value = 0u32;
            let mut i: u32 = 0;
            loop {
                let mut buf = [0];
                reader
                    .read_exact(&mut buf)
                    .map_err(|_| "Failed to read byte")?;
                let b = buf[0];

                if (b & 0x80) == 0 {
                    // This is the last byte.
                    value |= (b as u32) << i;
                    return Ok(value);
                } else {
                    value |= ((b & 0x7F) as u32) << i;
                    i += 7;
                    if i > 28 {
                        // Varint is too long.
                        return Err("Invalid varint: exceeds 5 bytes");
                    }
                }
            }
        };

        let mut test_buffer = Vec::new();

        // The original Java test loops up to Integer.MAX_VALUE with a small step.
        // Doing this for u32::MAX would take too long. We can achieve the same
        // goal by sampling a wide range of numbers with a large prime step.
        // This ensures we test values requiring 1, 2, 3, 4, and 5 bytes.
        let samples = (0..=u32::MAX).step_by(29999);

        for i in samples {
            // Write the test value into our buffer.
            write_unsigned_varint(i, &mut test_buffer).expect("Writing to vec should not fail");

            // --- Verification using the function under test ---
            // Wrap the buffer in a Cursor to make it readable.
            // Equivalent to `testData.flip()` and `testData.duplicate()`.
            let mut cursor1 = Cursor::new(&test_buffer);
            let actual = read_unsigned_varint(&mut cursor1)
                .expect("The function under test failed to read a valid varint");

            // --- Verification using the reference implementation ---
            let mut cursor2 = Cursor::new(&test_buffer);
            let expected = simple_read_impl(&mut cursor2)
                .expect("The simple reference implementation failed to read a valid varint");

            // The main assertion, equivalent to `assertEquals(expected, actual);`
            assert_eq!(expected, actual, "Mismatch for value: {}", i);

            // Equivalent to `testData.clear();`
            test_buffer.clear();
        }
    }

    #[test]
    fn test_invalid_varint() {
        // varlong encoding has one overflow byte
        let mut buf: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
        ];
        assert!(matches!(
            read_varint(&mut buf),
            Err(VarintError::VarintTooLong)
        ));
    }

    #[test]
    fn test_correctness_read_unsigned_varint64() {
        // The "simpleImplementation" from the Java test, converted to a Rust closure.
        // It takes anything that implements `io::Read` and returns a Result.
        let simple_read_impl = |reader: &mut dyn Read| -> Result<u64, &'static str> {
            let mut value = 0u64;
            let mut i: u64 = 0;
            loop {
                let mut buf = [0];
                reader
                    .read_exact(&mut buf)
                    .map_err(|_| "Failed to read byte")?;
                let b = buf[0];

                if (b & 0x80) == 0 {
                    // This is the last byte.
                    value |= (b as u64) << i;
                    return Ok(value);
                } else {
                    value |= ((b & 0x7F) as u64) << i;
                    i += 7;
                    if i > 63 {
                        // Varint is too long.
                        return Err("Invalid varint: exceeds 10 bytes");
                    }
                }
            }
        };

        let mut test_buffer = Vec::new();

        // --- Test Case Generation ---
        // Instead of a slow linear scan, we generate a list of critical values
        // to test. This is much faster and more effective.
        let mut test_values = vec![0, 1, u64::MAX];

        // Add boundary values around powers of 2, which are the most likely
        // places for encoding/decoding errors to occur.
        for n in 1..64 {
            let base = 1u64 << n;
            test_values.push(base.saturating_sub(1)); // e.g., 127 (0b0111_1111)
            test_values.push(base); // e.g., 128 (0b1000_0000)
            if let Some(plus_one) = base.checked_add(1) {
                test_values.push(plus_one); // e.g., 129
            }
        }

        // Sort and remove duplicates for a clean test run.
        test_values.sort();
        test_values.dedup();

        println!(
            "Testing {} critical u64 values for varint correctness...",
            test_values.len()
        );

        for i in test_values {
            // Write the test value into our buffer.
            write_unsigned_varint64(i, &mut test_buffer).expect("Writing to vec should not fail");

            // --- Verification using the function under test ---
            // Wrap the buffer in a Cursor to make it readable.
            // Equivalent to `testData.flip()` and `testData.duplicate()`.
            let mut cursor1 = Cursor::new(&test_buffer);
            let actual = read_unsigned_varint64(&mut cursor1)
                .expect("The function under test failed to read a valid varint");

            // --- Verification using the reference implementation ---
            let mut cursor2 = Cursor::new(&test_buffer);
            let expected = simple_read_impl(&mut cursor2)
                .expect("The simple reference implementation failed to read a valid varint");

            // The main assertion, equivalent to `assertEquals(expected, actual);`
            assert_eq!(expected, actual, "Mismatch for value: {}", i);

            // Equivalent to `testData.clear();`
            test_buffer.clear();
        }
    }

    // Helper function to assert that a value is serialized to the expected bytes,
    /// and can be deserialized back to the original value.
    ///
    /// This single function covers all the cases from the Java version because
    /// the Rust functions are generic over `io::Read` and `io::Write`.
    fn assert_unsigned_varint_serde(value: u32, expected_encoding: &[u8]) {
        // --- Test Serialization (Writing) ---

        // 1. Write the value to an in-memory buffer (Vec<u8>).
        //    `&mut Vec<u8>` implements `io::Write`.
        let mut buffer = Vec::new();
        write_unsigned_varint(value, &mut buffer).expect("Writing to a Vec should not fail");

        // 2. Assert that the encoded bytes match the expected output.
        //    This is equivalent to `assertArrayEquals(expectedEncoding, ...)`
        assert_eq!(
            expected_encoding,
            buffer.as_slice(),
            "Encoding mismatch for value {}",
            value
        );

        // --- Test Deserialization (Reading) ---

        // 3. Create a readable cursor from the bytes we just wrote.
        //    `Cursor<&Vec<u8>>` implements `io::Read`.
        let mut cursor = Cursor::new(&buffer);
        let decoded_value =
            read_unsigned_varint(&mut cursor).expect("Reading from a cursor should not fail");

        // 4. Assert that the decoded value matches the original.
        //    This is equivalent to `assertEquals(value, ...)`
        assert_eq!(
            value, decoded_value,
            "Decoded value mismatch for original value {}",
            value
        );
    }

    // Helper function to assert that a value is serialized to the expected bytes,
    /// and can be deserialized back to the original value.
    ///
    /// This single function covers all the cases from the Java version because
    /// the Rust functions are generic over `io::Read` and `io::Write`.
    fn assert_varint_serde(value: i32, expected_encoding: &[u8]) {
        // --- Test Serialization (Writing) ---

        // 1. Write the value to an in-memory buffer (Vec<u8>).
        //    `&mut Vec<u8>` implements `io::Write`.
        let mut buffer = Vec::new();
        write_varint(value, &mut buffer).expect("Writing to a Vec should not fail");

        // 2. Assert that the encoded bytes match the expected output.
        //    This is equivalent to `assertArrayEquals(expectedEncoding, ...)`
        assert_eq!(
            expected_encoding,
            buffer.as_slice(),
            "Encoding mismatch for value {}",
            value
        );

        // --- Test Deserialization (Reading) ---

        // 3. Create a readable cursor from the bytes we just wrote.
        //    `Cursor<&Vec<u8>>` implements `io::Read`.
        let mut cursor = Cursor::new(&buffer);
        let decoded_value =
            read_varint(&mut cursor).expect("Reading from a cursor should not fail");

        // 4. Assert that the decoded value matches the original.
        //    This is equivalent to `assertEquals(value, ...)`
        assert_eq!(
            value, decoded_value,
            "Decoded value mismatch for original value {}",
            value
        );
    }
}
