// Hashing
use hex;
use serde::{Serialize, Deserialize};
use bincode;




fn main() {
    use ring::digest;

    // 1. create a **String** that contains your name.
    let name: String = String::from("Chiao-Wei Hsu");

    // 2. convert it to bytes.
    let bytes: &[u8] = name.as_bytes();

    // 3. use SHA256 hash function in *ring* crate to compute the hash value of your name.
    let hash: digest::Digest = digest::digest(&digest::SHA256, bytes);
    // 4. use *hex* crate to encode hash value to hex format.
    let hex: String = hex::encode(hash);

    // 5. define a struct named **NameHash** and create an instance of this struct that contains both your name and the hex foramt hash.
    // 6. derive Debug trait on **NameHash**.
    // 7. derive Serialize and Deserialize trait on **NameHash** (see *serde* crate on *docs.rs*, also see [this](https://serde.rs/derive.html)).
    #[derive(Debug, Serialize, Deserialize)]
    struct NameHash {
        name: String,
        hash: String,
    }
    let name_hash: NameHash = NameHash {
        name: name,
        hash: hex,
    };

    // 8. serialize the **NameHash** instance into bytes using *bincode* crate.
    let serialized: Vec<u8> = bincode::serialize(&name_hash).unwrap();

    // 9. deserialize bytes back to the instance using *bincode* crate..unwrap();
    let deserialized: NameHash = bincode::deserialize(&serialized).unwrap();
    // 10. print on screen the serialized bytes and the deserialized instance using Debug format (hint: use "{:?}" instead of "{}").
    println!("{:?}", serialized);
    println!("{:?}", deserialized);

    }

// expected output
// [10, 0, 0, 0, 0, 0, 0, 0, 74, 111, 104, 110, 32, 83, 109, 105, 116, 104, 64, 0, 0, 0, 0, 0, 0, 0, 101, 102, 54, 49, 97, 53, 55, 57, 99, 57, 48, 55, 98, 98, 101, 100, 54, 55, 52, 99, 48, 100, 98, 99, 98, 99, 102, 55, 102, 55, 97, 102, 56, 102, 56, 53, 49, 53, 51, 56, 101, 101, 102, 55, 98, 56, 101, 53, 56, 99, 53, 98, 101, 101, 48, 98, 56, 99, 102, 100, 97, 99, 52, 97]
// NameHash { name: "John Smith", hash: "ef61a579c907bbed674c0dbcbcf7f7af8f851538eef7b8e58c5bee0b8cfdac4a" }