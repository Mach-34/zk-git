use hex::encode;
use sha1::{Digest, Sha1};
use std::{env, fs, os::unix::fs::MetadataExt, path::PathBuf, io::Write as IoWrite, fmt::Write};
use flate2::write::DeflateEncoder;

pub(crate) const SPACE: &[u8; 1] = b" ";

enum GitObject {
    Blob,
    Tree,
    Commit,
}

impl GitObject {
    fn as_str(&self) -> &str {
        match *self {
            GitObject::Blob => "blob",
            GitObject::Tree => "tree",
            GitObject::Commit => "commit",
        }
    }
}

fn mask_file_mode(mode: u32) -> u32 {
    match mode {
        0o100664 => 0o100644,
        0o100775 => 0o100755,
        _ => mode,
    }
}

fn main() {
    // let file = "src/main.rs";
    // let path = env::current_dir().unwrap().join(file);
    // // let hash = git_hash_object_blob(&path);
    // // println!("hash: {:x?}", &hash[..]);
    // // println!("mode: {:o}", get_file_mode(&path));
    // let entry = blob_git_tree_entry(&path);
    // println!("Entry: {entry}");
    let path = env::current_dir().unwrap().join("gittest/src/pewpew");
    let hash = build_git_tree_object(&path);
    println!("hash: {}", hex::encode(&hash));
    // assert_eq!(
    //     "25ef1f3d9781104304577434c49c7cb8eeaad4c2",
    //     hex::encode(&hash)
    // );
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
fn build_git_tree_object(path: &PathBuf) -> Vec<u8> {
    // get all files in the directory
    let files = get_directory_files(&path);
    let mut entries = Vec::<(String, String)>::new();
    for file in files {
        entries.push(blob_git_tree_entry(&file));
    }
    // order lexigraphically by file name
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    // funnel into ordered list of blob entries
    let entries: Vec<&String> = entries.iter().map(|(_, entry)| entry).collect();
    // build preimage of tree object
    let content = entries
        .iter()
        .fold(String::new(), |acc, entry| format!("{}\n{}", acc, *entry));
    let content = content[1..].to_string();
    println!("Content: {}", content);
    // compute the `git hash-object` sha1 hash of the tree object
    git_hash_object(GitObject::Tree, &content)
}

/**
 * Compute the `git hash-object` sha1 hash of a blob
 *
 * @param path The path to the file to hash
 * @return The sha1 hash of the file with blob header
 */
fn git_hash_object(obj: GitObject, content: &String) -> Vec<u8> {
    // get number of bytes in contents
    let length = content.as_bytes().len();
    // build preimage of blob as header + content
    // let preimage = format!("{} {}\0{}", obj.as_str(), length, content);
    // let mut preimage = String::new();
    // write!(&mut preimage, "{} {}\0{}", obj.as_str(), length, content).unwrap();
    // println!("preimage: {}", preimage);
    let mut preimage = Vec::<u8>::new();
    preimage.write_all(obj.as_str().as_bytes()).unwrap();
    preimage.write_all(&[b' ']).unwrap();
    preimage.write_all(&format!("{}", length).as_bytes()).unwrap();
    preimage.write_all(&[b'\0']).unwrap();
    preimage.write_all(content.as_bytes()).unwrap();
    // let string = String::from_utf8(preimage.clone());
    // println!("THE FULL THING: {:?}", string);
    // let compressed = Vec::<u8>::new();
    // let mut zlib = DeflateEncoder::new(compressed, flate2::Compression::default());
    // zlib.write_all(preimage.as_slice()).unwrap();
    // let compressed = zlib.finish().unwrap();
    // println!("Compressed: {:?}", compressed);
    // hash preimage
    let mut hasher = Sha1::new();
    hasher.update(preimage);
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
    // get file contents
    let content = fs::read_to_string(path).unwrap();
    // get compatible `git hash-object` sha1 hash of blob
    let hash = hex::encode(git_hash_object(GitObject::Blob, &content));
    // get filemode & name
    let mode = mask_file_mode(fs::metadata(&path).unwrap().mode());
    let name = *(&path.file_name().unwrap().to_str().unwrap());
    // build tree entry
    // let entry = format!("{mode:o} blob {hash}\t{name}");
    let mut preimage = Vec::<u8>::new();
    preimage.write_all(&format!("{mode:o}",).as_bytes()).unwrap();
    preimage.write_all(&[b' ']).unwrap();
    preimage.write_all("blob".as_bytes()).unwrap();
    preimage.write_all(&[b' ']).unwrap();
    preimage.write_all(hash.as_bytes()).unwrap();
    preimage.write_all(&"\u{0009}".as_bytes()).unwrap();
    // preimage.write_all(&[b' ']).unwrap();
    preimage.write_all(name.as_bytes()).unwrap();

    let string = String::from_utf8(preimage.clone()).unwrap();

    println!("preimage: {:}", string);

    (name.to_string(), string)
}
