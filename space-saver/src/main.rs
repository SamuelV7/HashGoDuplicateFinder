


use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs::File, fs};
use std::io::{Read, Write};
use std::thread;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FileHash{
    hash: String,
    path: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct FileInfo{
    count: u32,
    hash: String,
    // file_name: Vec<String>,
    path: Vec<String>,
}

fn file_hash_sha_256_buffered(file: &str) -> Option<String> {
    let f = File::open(file);
    match f {
        Ok(mut file) => {
            let mut hasher = Sha256::new();
            let mut buffer = [0; 4096*100];
            loop {
                let bytes_read = file.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            let hash = hasher.finalize();
            // println!("{:x}", hash.clone());
            Some(format!("{:x}", hash))
        },
        Err(_) => None,
    }
}

fn file_hash_sha256(file: &str) -> Option<String> {
    let f = File::open(file);
    match f {
        Ok(mut file) => {
            // let buffered_reader = std::io::BufReader::new(file);#
            let mut buffer =Vec::new();
            file.read(&mut buffer).unwrap();
            let mut hasher = Sha256::new();
            Digest::update(&mut hasher, buffer);
            let result = hasher.finalize();
            Some(format!("{:x}", result))
        },
        Err(_) => return None,
    };
    None
}

fn walk_directory(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    files.append(&mut walk_directory(path.to_str().unwrap()));
                } else {
                    files.push(path.to_str().unwrap().to_string());
                }
            },
            Err(_) => {continue;},
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
    let out = walk_directory(dir);
    
    // spawn thread to hash files using rayon and send to channel
    let hashing_thread = thread::spawn(move || {
        out.into_par_iter().for_each(|x| {
            let hash_optional = file_hash_sha_256_buffered(&x);
            match hash_optional {
                Some(hash) => {
                    let fh = FileHash {
                        hash,
                        path: x,
                    };
                    tx.clone().send(fh).unwrap();
                },
                None => {
                    println!("Error in hashing file: {}", x);
                }
            }
        });
    });

    // wait for thread to finish
    let store_thread = thread::spawn(move || {
        let map: std::collections::HashMap<String, Vec<FileHash>> = std::collections::HashMap::new();
        add_into_hash_map(&rx, map);
    });
    // join two threads
    let _st_r = store_thread.join();
    let _th_r = hashing_thread.join();

}

fn add_into_hash_map(rx: &std::sync::mpsc::Receiver<FileHash>, mut map: std::collections::HashMap<String, Vec<FileHash>>) {
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
                converting_hashmap_to_json(&map);
                break;
            }
        }            
    }
}

fn converting_hashmap_to_json(map: &std::collections::HashMap<String, Vec<FileHash>>) {
    let new_map : Vec<FileInfo> = map.iter().map(|(key, value)| {
        // get file name from path
        let item = FileInfo{
            // file_name: value.iter().map(|x| {
            //     let path = Path::new(&x.path);
            //     let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            //     file_name
            // }).collect(),
            count: value.len() as u32,
            hash: key.clone(),
            path: value.iter().map(|x| x.path.clone()).collect(),
        };
        item
    }).collect();
    let json = serde_json::to_string(&new_map).unwrap();
    let time_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
    let mut file = File::create(format!("{}-file-info.json", time_stamp)).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("File Saved At: {}", std::env::current_dir().unwrap().to_str().unwrap());
}


fn main() {
    let dir = "/Users/samuelvarghese/Downloads".to_string();
    channel_with_hashmap(&dir);
}
