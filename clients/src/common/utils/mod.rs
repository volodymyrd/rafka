use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self};

/// This module exposes low-level methods for reading/writing from byte streams or buffers.
pub mod byte_utils {
    use super::*;
    use std::io::Cursor;

    /// Read an unsigned 32-bit integer from a reader, advancing the position by 4 bytes.
    ///
    /// This function reads four bytes in big-endian order and returns the value
    /// as a signed 64-bit integer, which is the equivalent of Java's `long`.
    ///
    /// # Arguments
    ///
    /// * `reader` - The data source to read from, which must implement `std::io::Read`.
    ///
    /// # Returns
    ///
    /// The unsigned 32-bit integer read, as an `i64`, or an `io::Result` error if the read fails.
    pub fn read_unsigned_int<R: io::Read>(reader: &mut R) -> io::Result<i64> {
        Ok(reader.read_u32::<BigEndian>()? as i64)
    }

    /// Read an unsigned 32-bit integer from a specific position in a byte slice.
    ///
    /// This function reads four bytes from the given index in big-endian order and
    /// returns the value as a 64-bit signed integer, mirroring Java's `long` return type.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The byte slice to read from.
    /// * `index` - The starting index in the slice from which to read the integer.
    ///
    /// # Returns
    ///
    /// The unsigned 32-bit integer read, as an `i64`, or an `io::Result` error if the read fails.
    pub fn read_unsigned_int_from_pos(buffer: &[u8], index: usize) -> io::Result<i64> {
        // Ensure the index and length are valid before attempting to read.
        if buffer.len() < index + 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "index out of bounds",
            ));
        }

        // Create a cursor from the sub-slice starting at the given index.
        // This allows us to use standard `Read` trait methods.
        let mut reader = Cursor::new(&buffer[index..]);

        // Read a u32 (unsigned 32-bit integer) in big-endian format.
        // The `?` operator propagates any I/O error.
        let value = reader.read_u32::<BigEndian>()?;

        // Cast the u32 to an i64 to match the Java `long` return type.
        Ok(value as i64)
    }

    /// Read a big-endian signed 32-bit integer from a byte slice at a given offset.
    ///
    /// This function uses the `byteorder` crate to safely read four bytes from the
    /// specified position and convert them into a 32-bit signed integer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The byte slice to read from.
    /// * `offset` - The starting index (offset) in the slice from which to read.
    ///
    /// # Returns
    ///
    /// The signed 32-bit integer read, or an `io::Result` error if the read fails.
    pub fn read_int_be(buffer: &[u8], offset: usize) -> io::Result<i32> {
        // Check for out-of-bounds access.
        if buffer.len() < offset + 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "buffer is too short to read a 32-bit integer",
            ));
        }

        // Create a cursor to read from the byte slice starting at the offset.
        let mut reader = Cursor::new(&buffer[offset..]);

        // Use `read_i32` with `BigEndian` to perform the read.
        reader.read_i32::<BigEndian>()
    }

    /// Write the given 64-bit value as a 4-byte unsigned integer at a specific position.
    ///
    /// This function truncates the input `i64` to a 32-bit unsigned integer and writes it
    /// to the specified position in the buffer in big-endian byte order.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The mutable byte slice to write to.
    /// * `index` - The starting index (offset) in the slice at which to write.
    /// * `value` - The `i64` value to write.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating success or failure of the write operation.
    pub fn write_unsigned_int_from_pos(
        buffer: &mut [u8],
        index: usize,
        value: i64,
    ) -> io::Result<()> {
        // Check for out-of-bounds access.
        if buffer.len() < index + 4 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "buffer is too short to write a 32-bit integer",
            ));
        }

        // Create a mutable cursor from the sub-slice starting at the given index.
        let mut writer = Cursor::new(&mut buffer[index..]);

        // Write the value as a 32-bit unsigned integer in big-endian format.
        writer.write_i32::<BigEndian>(value as i32)
    }

    /// Write the given 64-bit signed integer as a 4-byte unsigned integer. Overflow is ignored.
    ///
    /// This function truncates the input `i64` to a 32-bit signed integer and writes it
    /// to the writer in big-endian byte order, mimicking the behavior of the Java
    /// `ByteBuffer.putInt()` method with a long value.
    ///
    /// # Arguments
    ///
    /// * `writer` - The data sink to write to, which must implement `std::io::Write`.
    /// * `value` - The `i64` value to write.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating success or failure of the write operation.
    pub fn write_unsigned_int<W: io::Write>(writer: &mut W, value: i64) -> io::Result<()> {
        writer.write_i32::<BigEndian>(value as i32)
    }

    /// Writes an unsigned 32-bit integer in little-endian format to the writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The writer to write to.
    /// * `value` - The value to write.
    pub fn write_unsigned_int_le(writer: &mut impl io::Write, value: i32) -> io::Result<()> {
        writer.write_all(&value.to_le_bytes())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::Cursor;

        #[test]
        fn test_read_and_write_unsigned_int() {
            // Use a Cursor to simulate a ByteBuffer for in-memory I/O.
            let mut buffer = Cursor::new([0u8; 4]);

            // The value to write is a u32, directly corresponding to the 4 bytes.
            let write_value: i64 = 133444;

            // Write the value to the buffer. `unwrap()` is used for test simplicity.
            write_unsigned_int(&mut buffer, write_value).unwrap();

            // Reset the buffer's position to the beginning for reading.
            buffer.set_position(0);

            // Read the value back from the buffer.
            let read_value = read_unsigned_int(&mut buffer).unwrap();

            // Assert that the written and read values are identical.
            assert_eq!(write_value, read_value);
        }

        #[test]
        fn test_read_int() {
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

            // Create a buffer with enough space for all values.
            let mut buffer = vec![0u8; 4 * values.len()];
            let mut cursor = Cursor::new(&mut buffer);

            for (i, &value) in values.iter().enumerate() {
                // Write the value into the buffer at the correct position.
                cursor.set_position((i * 4) as u64);
                cursor.write_i32::<BigEndian>(value).unwrap();

                // Read the value back and assert it matches.
                let read_value = read_int_be(cursor.get_ref(), i * 4).unwrap();
                assert_eq!(value, read_value, "Written value should match read value.");
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
            let value2 = -185339151; // represents 0xf4f3f2f1 in i32
            let mut buffer2 = Cursor::new(Vec::new());
            write_unsigned_int_le(&mut buffer2, value2).unwrap();

            // Assert the written bytes are as expected
            assert_eq!(buffer2.into_inner(), vec![0xf1, 0xf2, 0xf3, 0xf4]);
        }
    }
}
