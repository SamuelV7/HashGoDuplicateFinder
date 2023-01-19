use std::fs::File;
use std::io::Read;
use sha2::{Sha256, Digest, digest::DynDigest};

fn file_hash_sha256(file: &str) -> String {
    let mut f = File::open(file).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, &buffer);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    println!("Hello, world!");
}
