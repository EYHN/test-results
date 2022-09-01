#[test]
fn save_test() {
    save!("output.txt", [0x31, 0x32, 0x33]);
    let output_dir: std::path::PathBuf = save_dir!("test-output");
    dbg!(output_dir);
}
