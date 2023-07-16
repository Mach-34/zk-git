use sha1::{Digest, Sha1};
use std::{
    env,
    fs,
    path::PathBuf,
    os::unix::fs::{MetadataExt}
};
use hex::encode;

fn main() {
    // let file = "src/main.rs";
    // let path = env::current_dir().unwrap().join(file);
    // // let hash = git_hash_object_blob(&path);
    // // println!("hash: {:x?}", &hash[..]);
    // // println!("mode: {:o}", get_file_mode(&path));
    // let entry = blob_git_tree_entry(&path);
    // println!("Entry: {entry}");
    let path = env::current_dir().unwrap().join("gittest/src");
    build_git_tree_object(&path);
}

/**
 * Given a path to a folder, return a vector of all files in that folder
 */
fn get_directory_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for file in fs::read_dir(&path).unwrap() {
        files.push(file.unwrap().path());
    }
    files
}

/**
 * Given a 
 * @todo: recursive subfolder building (should call itself?)
 */
fn build_git_tree_object(path: &PathBuf) {
    // get all files in the directory
    let files = get_directory_files(&path);
    let mut entries = Vec::<(String, String)>::new();
    for file in files {
        entries.push(blob_git_tree_entry(&file));
    }
    // order lexigraphically by file name
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let entries: Vec<&String> = entries.iter().map(|(_, entry)| entry).collect();
    println!("files: {entries:?}");

}

/**
 * Compute the `git hash-object` sha1 hash of a blob
 * 
 * @param path The path to the file to hash
 * @return The sha1 hash of the file with blob header
 */
fn git_hash_object_blob(path: &PathBuf) -> Vec<u8> {
    // get file contents
    let content = fs::read_to_string(path).unwrap();
    // get number of bytes in contents
    let length = content.as_bytes().len();
    // build preimage of blob as header + content
    let preimage = format!("blob {}\0{}", length, content);
    // hash preimage
    let mut hasher = Sha1::new();
    hasher.update(preimage.as_bytes());
    // return compatible `git hash-object` sha1 hash of blob
    hasher.finalize().as_slice().to_owned()
}


/**
 * Build the blob entry for a tree object
 *  - file mode + git object-hash + file name
 * 
 * @param path The path to the file to build a tree entry for
 * @return 
 *   - name of file
 *   - string representing the blob entry in tree object
 */
fn blob_git_tree_entry(path: &PathBuf) -> (String, String) {
    // get compatible `git hash-object` sha1 hash of blob
    let hash = hex::encode(git_hash_object_blob(&path));
    // get filemode & name
    let mode = fs::metadata(&path).unwrap().mode();
    let name = *(&path.file_name().unwrap().to_str().unwrap());
    // build tree entry
    let entry = format!("{mode:o} blob {hash}\t{name}");
    (name.to_string(), entry)
}