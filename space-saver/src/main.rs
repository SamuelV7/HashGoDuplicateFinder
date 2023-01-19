use std::{fs::File, fs};
use std::io::Read;
use std::thread;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct FileHash{
    hash: String,
    path: String,
}

fn file_hash_sha256(file: &str) -> String {
    let mut f = File::open(file).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, &buffer);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn walk_directory(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files.append(&mut walk_directory(&path.to_str().unwrap()));
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}

fn print_hash_map(map: &std::collections::HashMap<String, Vec<FileHash>>) {
    for (key, value) in map {
        let file_count = value.len();
        if file_count > 1 {
            println!("Key: {}, Count: {}", key, file_count);
        }
    }
}

fn channel_with_hashmap(dir: &str) {
    // create channel to send hashes
    let (tx, rx) = std::sync::mpsc::sync_channel(50);
    let out = walk_directory(&dir);
    
    // spawn thread to hash files using rayon and send to channel
    let hashing_thread = thread::spawn(move || {
        out.into_par_iter().for_each(|x| {
            let hash = file_hash_sha256(&x);
            let fh = FileHash {
                hash: hash.clone(),
                path: x.clone(),
            };
            tx.clone().send(fh).unwrap();
        });
    });

    // wait for thread to finish
    let store_thread = thread::spawn(move || {
        let map: std::collections::HashMap<String, Vec<FileHash>> = std::collections::HashMap::new();
        add_into_hash_map(rx, map);
    });
    // join two threads
    let _st_r = store_thread.join();
    let _th_r = hashing_thread.join();

}

fn add_into_hash_map(rx: std::sync::mpsc::Receiver<FileHash>, mut map: std::collections::HashMap<String, Vec<FileHash>>) {
    loop {
        let hashes = rx.recv();
        match hashes {
            Ok(hash) => {
                // add to hashmap if hash exists to its vector
                if map.contains_key(&hash.hash) {
                    map.get_mut(&hash.hash).unwrap().push(hash);
                }
                else{
                    // else create a new vector and add to hashmap
                    map.insert(hash.hash.clone(), vec![hash]);
                }
            },
            Err(_) => {
                println!("There is no more data to read from the channel");
                print_hash_map(&map);
                break;
            }
        }            
    }
}

fn main() {
    let dir = "/Users/samuelvarghese/Documents".to_string();
    // let out = walk_directory(&dir);
    channel_with_hashmap(&dir);
    println!("Hello, world!");
}
