use bincode::Options;
use std::mem;
use serde::{Serialize, Serializer, ser::SerializeSeq};

fn main() {
    let x = MyU64(0u64);
    println!("Serializing with bincode");
    let encoder = bincode::DefaultOptions::new();
    encoder.serialize(&x).unwrap();

    println!("Serializing with serde_json");
    serde_json::to_vec(&x).unwrap();
}

struct MyU64(pub u64);

impl Serialize for MyU64 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        eprintln!("Serializing {:0x}", self.0);
        let bytes = self.0.to_be_bytes();
        let mut seq = serializer.serialize_seq(Some(mem::size_of::<u64>()))?;
        for byte in bytes {
            seq.serialize_element(&byte)?;
        }
        seq.end()
    }
}
