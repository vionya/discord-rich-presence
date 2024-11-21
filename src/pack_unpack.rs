use crate::Error;
use std::convert::TryInto;
use std::mem::size_of;

// Re-implement some packing methods in Rust
pub fn pack(opcode: u32, data_len: u32) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();

    for byte_array in &[opcode.to_le_bytes(), data_len.to_le_bytes()] {
        bytes.extend_from_slice(byte_array);
    }

    Ok(bytes)
}

pub fn unpack(data: Vec<u8>) -> Result<(u32, u32), Error> {
    let data = data.as_slice();
    let (opcode, header) = data.split_at(size_of::<u32>());

    let opcode = u32::from_le_bytes(opcode.try_into().map_err(|_err| Error::Unpack {
        expected: size_of::<u32>(),
        received: opcode.len(),
    })?);
    let header = u32::from_le_bytes(header.try_into().map_err(|_err| Error::Unpack {
        expected: size_of::<u32>(),
        received: header.len(),
    })?);

    Ok((opcode, header))
}
