// src/lib.rs

// Declare all modules in the library.
pub mod ast;
pub mod environment;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod value;
pub mod error;

// Expose the key components for external use (e.g., by main.rs or tests).
pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::Interpreter;
pub use value::Value;
