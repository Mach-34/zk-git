use flate2::write::DeflateEncoder;
use hex::encode;
use sha1::{Digest, Sha1};
use std::{env, fmt::Write, fs, io::Write as IoWrite, os::unix::fs::MetadataExt, path::PathBuf};

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
    println!("hash:     {}", hex::encode(&hash));
    println!("expected: {}", "25ef1f3d9781104304577434c49c7cb8eeaad4c2");
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
    // println!("content: {}", content);
    let transformed = transform(&content);
    // compute the `git hash-object` sha1 hash of the tree object
    // git_hash_object(GitObject::Tree, &content)
    let expected = "tree 155100644 copy.rs�NŞX�>��%�I]�3��Lr100755 executable.sh6Օ��l/	K��F��cz��100644 lol1.txt�e��o��w��f���d��100644 ppppppppppp.txtE�^/��k���y�ầ1]V�8";
    let mut hasher = Sha1::new();
    hasher.update(expected.as_bytes());
    hasher.finalize().as_slice().to_owned()
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
    let mut preimage = Vec::<u8>::new();
    preimage.write_all(obj.as_str().as_bytes()).unwrap();
    preimage.write_all(&[b' ']).unwrap();
    preimage
        .write_all(&format!("{}", length).as_bytes())
        .unwrap();
    preimage.write_all(&[b'\0']).unwrap();
    preimage.write_all(content.as_bytes()).unwrap();
    // println!(
    //     "preimage: {:?}",
    //     String::from_utf8(preimage.clone()).unwrap()
    // );
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
    let mut preimage = Vec::<u8>::new();
    preimage
        .write_all(&format!("{mode:o}",).as_bytes())
        .unwrap();
    preimage.write_all(&[b' ']).unwrap();
    preimage.write_all("blob".as_bytes()).unwrap();
    preimage.write_all(&[b' ']).unwrap();
    preimage.write_all(hash.as_bytes()).unwrap();
    preimage.write_all(&"\u{0009}".as_bytes()).unwrap();
    preimage.write_all(name.as_bytes()).unwrap();
    // preimage
    //     .write_all(&format!("{mode:o}",).as_bytes())
    //     .unwrap();
    // preimage.write_all(&[b' ']).unwrap();
    // preimage.write_all(name.as_bytes()).unwrap();
    // preimage.write_all(&[b'\0']).unwrap();
    // preimage.write_all(hash.as_bytes()).unwrap();
    // preimage.write_all(&[b'\n']).unwrap();

    let string = String::from_utf8(preimage.clone()).unwrap();

    // println!("preimage: {:}", string);

    (name.to_string(), string)
}

pub fn transform(inp: &String) -> String {
    let fields: Vec<&str> = inp.split_whitespace().collect();
    let mut formatted = String::new();
    for i in (0..fields.len()).step_by(4) {
        let field1 = fields.get(i);
        let field3 = fields.get(i + 2);
        let field4 = fields.get(i + 3);

        if let (Some(field1), Some(field3), Some(field4)) = (field1, field3, field4) {
            let bytes = hex::decode(field3).unwrap();
            let decoded = qqq(&bytes[..]).unwrap();
            let hashed = String::from_utf8(decoded.to_vec()).unwrap();
            println!("Hashed: {}", hashed);
            // let x = patsplit(field3);
            // let hashed = process_elements(&x);
            // formatted = format!("{}{} {}\0{}", formatted, field1, field4, hashed);
        }
    }
    let result = format!("tree {}\0{}", formatted.len(), formatted);
    println!("result: {}", result);
    result
}

fn patsplit(asha: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    for i in (0..asha.len()).step_by(2) {
        let chunk = asha[i..i + 2].to_string();
        chunks.push(chunk);
    }
    chunks
}

fn process_elements(x: &[String]) -> String {
    let mut h = String::new();
    for j in x {
        // let decimal_number = u8::from_str_radix(&j, 16).unwrap() as char;
        // println!("decimal_number: {:?}", decimal_number);
        // h.push(decimal_number)
        let decimal_number = &[u8::from_str_radix(&j, 16).unwrap()];
        let character = String::from_utf8_lossy(decimal_number);
        println!("char: {:?}", character);
        h = format!("{}{}", h, character);
    }
    h
}

pub fn qqq(input: &[u8]) -> Result<[u8; 20], &str> {
    let mut output = [0u8; 20];
    for (cursor, xs) in input.iter().enumerate() {
        println!("XS: {}", xs);
        let incoming = match xs {
            48 ..= 57 => xs - 48,
            97 ..= 102 => xs - 97 + 10,
            65 ..= 70 => xs - 65 + 10,
            _ => return Err("bad")
        };
        let to_shift = ((1 + cursor) & 1) << 2;
        output[cursor >> 1] |= incoming << to_shift;
    }
    Ok(output)
}