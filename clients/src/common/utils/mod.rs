/// This module exposes low-level methods for reading/writing from byte streams or buffers.
pub mod byte_utils {
    use std::io::{self};

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
    /// ```
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
        let bytes = value.to_be_bytes();
        buffer[index..index + 4].copy_from_slice(&bytes);
    }

    /// Writes a 32-bit unsigned integer to a buffer, advancing the buffer's position by 4 bytes.
    ///
    /// This function is the inverse of `read_unsigned_int`. It takes a `u32` value, converts it
    /// to a 4-byte big-endian representation, and writes those bytes to the given buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable reference to a type that implements the `std::io::Write` trait.
    /// * `value` - The unsigned 32-bit integer to write.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to write the 4 bytes to the buffer.
    pub fn write_unsigned_int(buffer: &mut dyn io::Write, value: u32) {
        let bytes = value.to_be_bytes();
        buffer.write_all(&bytes).unwrap();
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
        let bytes = value.to_be_bytes();
        buffer[index..index + 4].copy_from_slice(&bytes);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::Cursor;

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
    }
}
