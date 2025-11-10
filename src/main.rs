// main.rs
// This file controls the overall compilation pipeline:
// 1) Read the .lol input file
// 2) Run the lexer + parser to build an AST
// 3) Run semantic checks (optional for this phase but included)
// 4) Convert the AST to HTML
// 5) Write the HTML to disk and optionally open in browser

use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod lexer;
mod parser;
mod semantic;
mod htmlgen;
mod error;
mod token;
mod ast;

use parser::{Parser, SyntaxAnalyzer};
use semantic::Analyzer;
use htmlgen::HtmlGen;
use error::Result;

/// Opens the generated HTML file in a browser (Windows/Mac support).
fn open_in_browser(out_path: &PathBuf) {
    // Convert path to an absolute path
    let abs = fs::canonicalize(out_path).unwrap_or(out_path.clone());
    let mut s = abs.to_string_lossy().to_string();

    // Remove Windows "\\?\" prefix if present
    if s.starts_with(r"\\?\") {
        s = s.trim_start_matches(r"\\?\").to_string();
    }

    // Convert path to a `file:///` URL (browsers require this format)
    let file_url = format!("file:///{}", s.replace('\\', "/"));

    // Launch default browser depending on OS
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/C", "start", "", &file_url]).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open")
            .args(["-a", "Google Chrome", &file_url])
            .spawn()
            .or_else(|_| Command::new("open").arg(&file_url).spawn());
    }
}

fn main() -> Result<()> {
    // Get input file path from command line.
    // Example: cargo run -- src/test.lol
    let input = std::env::args()
        .nth(1)
        .expect("Usage: lolmarkdownn <file.lol>");

    // Read the entire .lol program as text
    let source = fs::read_to_string(&input)
        .expect("Failed to read input file");

    // 1) LEX + PARSE → produces AST
    let mut parser = Parser::new(&source)?;
    parser.parse_lolcode()?;      // fills parser.ast

    // 2) SEMANTIC ANALYSIS → validate AST (e.g., variable checks)
    let mut analyzer = Analyzer::new(&parser.ast);
    let checked_ast = analyzer.check()?; // returns validated AST

    // 3) HTML GENERATION → convert AST → HTML string
    let mut html_gen = HtmlGen::new();
    let html = html_gen.generate(&checked_ast);

    // Create output path by changing .lol → .html
    let mut out_path = PathBuf::from(&input);
    out_path.set_extension("html");

    // Write generated HTML to disk
    fs::write(&out_path, html).expect("write html");

    println!("✅ Generated: {}", out_path.display());

    // Automatically open the HTML file (optional)
    open_in_browser(&out_path);

    Ok(())
}
