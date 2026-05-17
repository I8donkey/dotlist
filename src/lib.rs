pub mod parser;
pub mod cli;
pub mod ide;

#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(feature = "nodejs")]
pub mod nodejs_bindings;

#[cfg(not(any(feature = "python", feature = "nodejs")))]
pub mod c_ffi;

// Rust 原生友好接口
pub mod rust_api;
