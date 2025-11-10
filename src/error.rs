// error.rs
// This file defines all error types used in the compiler.
// We separate errors based on which phase produced them:
// - Lexical: bad tokens
// - Syntax: grammar mismatch
// - Semantic: variable scope issues

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LolError {
    // Produced in the lexer when we see an invalid or unknown token.
    Lexical { line: usize, col: usize, msg: String },

    // Produced in the parser when the token does not match the grammar.
    Syntax  { expected: String, found: String },

    // Produced during static scope checking (e.g., variable not defined).
    Semantic(String),
}

impl fmt::Display for LolError {
    // Formats the error message in a readable form.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LolError::Lexical { line, col, msg } =>
                write!(f, "Lexical error at line {}, col {}: {}", line, col, msg),
            LolError::Syntax { expected, found } =>
                write!(f, "Syntax error: expected {}, found {}", expected, found),
            LolError::Semantic(s) =>
                write!(f, "Static semantic error: {}", s),
        }
    }
}

impl Error for LolError {}

// Simple Result alias so functions can return Result<T> instead of writing the full type.
pub type Result<T> = std::result::Result<T, LolError>;
