pub mod ast;
pub mod parser;

// Re-exporta os tipos principais da AST para todo o workspace do Navoa
pub use ast::{Statement, Expr, BinaryOp};
pub use parser::*;

