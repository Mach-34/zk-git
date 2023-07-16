#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use std::{fs, path::PathBuf};

use risc0_zkvm::guest::env;
use sha1::{Digest, Sha1};

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

risc0_zkvm::guest::entry!(main);

fn build_commit_hash(author: String, committer: String, parent_hash: String, tree_hash: String) {
    let content = vec![tree_hash, parent_hash, author, committer]
        .iter()
        .fold(String::new(), |acc, entry| format!("{}\n{}", acc, *entry));
    let hash = git_hash_object(GitObject::Commit, &content);
    println!("Hash: {}", hex::encode(hash))
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
fn blob_git_tree_entry(content: String, name: String, mode: u32) -> (String, String) {
    // get file contents
    // get compatible `git hash-object` sha1 hash of blob
    let hash = hex::encode(git_hash_object(GitObject::Blob, &content));
    // build tree entry
    let entry = format!("{mode:o} blob {hash}\t{name}");
    (name, entry)
}

/**
 * Given a
 * @todo: recursive subfolder building (should call itself?)
 */
fn build_git_tree_object(files: Vec<(String, String, u32)>) -> Vec<u8> {
    let mut entries = Vec::<(String, String)>::new();
    for file in files {
        let (content, name, mode) = file;
        entries.push(blob_git_tree_entry(content, name, mode));
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
    let preimage = format!("{} {}\0{}", obj.as_str(), length, content);
    // hash preimage
    let mut hasher = Sha1::new();
    hasher.update(preimage.as_bytes());
    // return compatible `git hash-object` sha1 hash of blob
    hasher.finalize().as_slice().to_owned()
}

// Pewpew 25ef1f3d9781104304577434c49c7cb8eeaad4c2
pub fn main() {
    // Hardcoded commit inputs
    let author: String = String::from("author Jack Gilcrest <me@jp4g.llc> 1689512391 -0400");
    let commiter: String = String::from("committer Jack Gilcrest <me@jp4g.llc> 1689512391 -0400");
    let parent_hash: String = String::from("parent 5527a59c126ec25a1d71ea6caa14f743438f97d9");
    let correct_tree_hash: String = String::from("tree 8fb675c4168409c76d307a9669747ac0027bb6ee");
    // let data: Vec<(String, String, u32)> = env::read();
    // let tree_hash = build_git_tree_object(data);
    build_commit_hash(author, commiter, parent_hash, correct_tree_hash);
    let place_holder_data = "Placeholder data";
    let length = place_holder_data.as_bytes().len();
    let preimage = format!("{} {}\0{}", "blob", length, place_holder_data);
    let mut hasher = Sha1::new();
    hasher.update(preimage.as_bytes());
    let hash = hasher.finalize().as_slice().to_owned();
    env::commit(&hex::encode(hash));
}
