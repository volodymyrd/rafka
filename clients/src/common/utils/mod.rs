use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self};

/// This module exposes low-level methods for reading/writing from byte streams or buffers.
pub mod byte_utils {
    use super::*;

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
    }
}
