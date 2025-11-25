// src/lib.rs

// Declare all modules in the library.
pub mod ast;
pub mod environment;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod native;
pub mod parser;
pub mod token;
pub mod value; // Add this line

// Expose the key components for external use (e.g., by main.rs or tests).
pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use parser::Parser;
pub use value::Value;
