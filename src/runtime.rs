use std::{
    fs,
    io::Write,
    path::{Path},
};

use crate::workspace::get_cargo_workspace;

pub fn save(name: &str, buffer: &[u8], file_path: &str, manifest_dir: &str) {
    let root = get_cargo_workspace(manifest_dir);
    let base = Path::new(file_path).parent().unwrap();
    let result_filename = root.join(base).join("test_results").join(name);

    fs::create_dir_all(result_filename.parent().unwrap()).unwrap();

    let mut file = std::fs::File::create(&result_filename).unwrap();
    file.write_all(buffer).unwrap();

    println!("Save Test Result: {}", result_filename.to_str().unwrap())
}
