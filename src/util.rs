use std::array::TryFromSliceError;
use std::convert::TryInto;

// Re-implement some packing methods in Rust
pub fn pack(opcode: u32, data_len: u32) -> Vec<u8> {
    let mut bytes = Vec::new();

    for byte_array in &[opcode.to_le_bytes(), data_len.to_le_bytes()] {
        bytes.extend_from_slice(byte_array);
    }

    bytes
}

pub fn unpack(data: Vec<u8>) -> Result<(u32, u32), TryFromSliceError> {
    let data = data.as_slice();
    let (opcode, header) = data.split_at(std::mem::size_of::<u32>());

    let opcode = u32::from_le_bytes(opcode.try_into()?);
    let header = u32::from_le_bytes(header.try_into()?);

    Ok((opcode, header))
}

#[doc(hidden)]
#[macro_export]
macro_rules! cmd {
    ($cmd:ident, $args:tt) => {
        {
            // Build the JSON for a command
            let mut data = serde_json::json!({
                "cmd": stringify!($cmd),
                "args": serde_json::json!($args),
                "nonce": uuid::Uuid::new_v4().to_string()
            });

            // The args object is dynamic, so we filter out nulls
            let mut map = data["args"].as_object().unwrap().to_owned();
            map.retain(|_, v| !v.is_null());

            // Then we replace args with the filtered map
            data["args"] = serde_json::Value::from(map);
            data
        }
    }
}

// pub fn jsonify_array<T: ToString>(array: &[T]) -> String {
//     array
//         .iter()
//         .map(|s| s.to_string())
//         .collect::<Vec<String>>()
//         .join(", ")
// }
