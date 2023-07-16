use sha1::{Digest, Sha1};
use std::{
    env,
    fs,
    path::PathBuf,
    os::unix::fs::{MetadataExt}
};
use hex::encode;

fn main() {
    let file = "src/main.rs";
    let path = env::current_dir().unwrap().join(file);
    // let hash = git_hash_object_blob(&path);
    // println!("hash: {:x?}", &hash[..]);
    // println!("mode: {:o}", get_file_mode(&path));
    let entry = make_git_tree_entry(&path);
    println!("Entry: {entry}");
}

fn git_hash_object_blob(path: &PathBuf) -> Vec<u8> {
    let content = fs::read_to_string(path).unwrap();
    let length = content.as_bytes().len();
    let preimage = format!("blob {}\0{}", length, content);
    let mut hasher = Sha1::new();
    hasher.update(preimage.as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn make_git_tree_entry(path: &PathBuf) -> String {
    let hash = hex::encode(git_hash_object_blob(&path));
    let mode = get_file_mode(&path);
    let name = *(&path.file_name().unwrap().to_str().unwrap());
    format!("{mode:o} blob {hash}\t{name}")
}

fn get_file_mode(path: &PathBuf) -> u32 {
    fs::metadata(path).unwrap().mode()
}
