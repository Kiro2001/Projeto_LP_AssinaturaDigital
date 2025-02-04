use sha3::{Sha3_256, Digest};

pub fn hash_content(content: &Vec<u8>) -> Vec<u8>{
    let mut hasher = Sha3_256::new();
    hasher.update(content);
    hasher.finalize().to_vec()
}