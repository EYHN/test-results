//! # test-results
//! 
//! A series of utility macros for outputting testing results.
//! 
//! # macros
//! 
//! ## `save!(name: &str, value: &u8)`
//! 
//! Save the value as test results.
//! 
//! example:
//! 
//! ```no_run
//! #[test]
//! fn test_hello_world() {
//!     test_results::save!("output.txt", [0x31, 0x32, 0x33]);
//! }
//! ```
//! 
//! Run the test and the file will be saved in the `test_results` folder in the same directory as the source files.
//! 
//! ```text
//! ├─ test.rs
//! └─ test_results
//!     └─ output.txt
//! ```
//! 
//! Optionally, you can add `test_results/` to the `.gitignore` file.
//! 
//! ```text
//! echo 'test_results/' >> .gitignore
//! ```
mod workspace;

mod runtime;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;

#[doc(hidden)]
pub mod _macro_support {
    pub use crate::runtime::*;
}
