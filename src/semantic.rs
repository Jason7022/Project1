use crate::ast::Node;
use crate::error::Result;

/// The Analyzer is responsible for semantic checks.
/// For Phase 2, we are not enforcing many rules yet.
/// It mainly passes the AST forward untouched.
pub struct Analyzer<'a> {
    // We borrow the AST produced by the parser
    ast: &'a [Node],
}

impl<'a> Analyzer<'a> {
    /// Store a reference to the AST that we will check.
    pub fn new(ast: &'a [Node]) -> Self {
        Self { ast }
    }

    /// This function is where semantic checks would normally happen.
    /// Examples of semantic checks (not implemented yet):
    /// - Making sure variables are defined before they are used.
    /// - Ensuring TITLE only appears inside HEAD.
    /// - Making sure LIST items are inside LIST blocks.
    ///
    /// For now, we simply return a clone of the AST.
    pub fn check(&mut self) -> Result<Vec<Node>> {
        // Later we can modify nodes here before HTML generation.
        Ok(self.ast.to_vec())
    }
}
