// Android JNI live path is exported only from `pkg2230`.
// `ffi.rs` is kept in-tree as a legacy/reference file and is not part of the
// current Rust module export chain. Do not treat the two files as equal live paths.
pub mod pkg2230;

pub use pkg2230::*;
