//! Rsppression - A high-performance expression evaluation engine
//!
//! This crate provides a fast and lightweight expression evaluation engine
//! that supports both syntax tree interpretation and bytecode virtual machine execution.

pub mod chunk;
pub mod environment;
pub mod error;
pub mod expr;
pub mod field;
pub mod functions;
pub mod ir;
pub mod parser;
pub mod runner;
pub mod values;
pub mod visitors;
pub mod vm;

pub use chunk::Chunk;
pub use environment::{DefaultEnvironment, Environment};
pub use error::{RspError, RspResult};
pub use field::Field;
pub use parser::{Parser, Scanner, Token, TokenType};
pub use runner::{ExecuteMode, RspRunner};
pub use values::Value;
