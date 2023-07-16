// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    default_executor_from_elf,
    serde::{from_slice, to_vec},
    ExecutorEnv,
};
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

fn collect_files() -> Vec<(String, String, u32)> {
    let mut files = Vec::new();
    let path = std::env::current_dir().unwrap().join("gittest/src/pewpew");
    for file in std::fs::read_dir(&path).unwrap() {
        let file_path = file.unwrap().path();
        let content = std::fs::read_to_string(&file_path).unwrap();
        // Get file mode and name
        let name = *(&file_path.file_name().unwrap().to_str().unwrap());
        let mode = mask_file_mode(fs::metadata(&file_path).unwrap().mode());
        files.push((content, name.to_string(), mode));
    }
    files
}

fn mask_file_mode(mode: u32) -> u32 {
    match mode {
        0o100664 => 0o100644,
        0o100775 => 0o100755,
        _ => mode,
    }
}

fn main() {
    let files = collect_files();

    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&files).unwrap())
        .build()
        .unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, METHOD_NAME_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(METHOD_NAME_ID).unwrap();

    // let hash: String = from_slice(&receipt.journal).unwrap();
    // println!("Host: {}", hash)
}
