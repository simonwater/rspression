//! Loxpression - A high-performance expression evaluation engine
//!
//! This crate provides a fast and lightweight expression evaluation engine
//! that supports both syntax tree interpretation and bytecode virtual machine execution.

pub mod chunk;
pub mod environment;
pub mod error;
pub mod expr;
pub mod parser;
pub mod runner;
pub mod scanner;
pub mod token;
pub mod value;
pub mod visitor;
pub mod vm;

pub use environment::{DefaultEnvironment, Environment};
pub use error::{LoxError, LoxResult};
pub use runner::LoxRunner;
pub use value::Value;
