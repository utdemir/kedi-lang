#[cfg(feature = "codegen-wasm")]
pub mod codegen_wasm;

#[cfg(feature = "codegen-js")]
pub mod codegen_js;

pub mod interpreter;

pub mod codegen;
pub mod error;
pub mod parser;
pub mod phase;
pub mod renamer;
pub mod runner;
pub mod simplifier;
pub mod util;

mod scratchpad;
