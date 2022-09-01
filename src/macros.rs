/// Save the value as test results
/// 
/// Example:
/// 
/// ```ignore
/// let png_data: &[u8] = some_rendering_logic().encode_to_png();
/// test_results::save!("output.png", png_data);
/// ```
/// 
/// The file will be saved in the `test_results` folder in the same directory as the source files.
/// 
/// ```text
/// ├─ test.rs
/// └─ test_results
///     └─ output.txt
/// ```
#[macro_export]
macro_rules! save {
    ($name: expr, $value:expr) => {
        $crate::_macro_support::save(
            $name.into(),
            &$value,
            file!(),
            env!("CARGO_MANIFEST_DIR")
        )
    };
}

/// Save a test results folder
/// 
/// Example:
/// 
/// ```ignore
/// let output_dir: PathBuf = test_results::save_dir!("test-output");
/// ```
/// 
/// The folder will be saved in the `test_results` folder in the same directory as the source files.
/// 
/// ```text
/// ├─ test.rs
/// └─ test_results
///     └─ test-output
/// ```
#[macro_export]
macro_rules! save_dir {
    ($name: expr) => {
        $crate::_macro_support::save_dir(
            $name.into(),
            file!(),
            env!("CARGO_MANIFEST_DIR")
        )
    };
}
