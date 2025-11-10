// token.rs
// This file defines the tokens and keywords used by the lexer and parser.
// Tokens are the smallest meaningful pieces of the language.

/// All keywords in LOL code.
/// We store them in an enum so the parser can match on them easily.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kw {
    Hai, Kthxbye,      // program start / end
    OBTW, TLDR,        // comments
    Maek, Gimmeh, Head, Title, Paragraf, OIC,   // structural tags
    Bold, Italics, Newline, Soundz, Vidz,       // formatting or media
    List, Item,                                // lists
    Lemme, See,                                // variable use
    I, Haz, It, Iz,                            // variable definition
    Mkay,                                       // closing marker
}

/// Tokens are what the lexer outputs to the parser.
/// - Hash: `#` indicates the start of a command or block
/// - Word: alphabetic/identifier text
/// - Text: punctuation or whitespace text
/// - Kw: recognized keyword
/// - Eof: end of input
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Hash,
    Word(String),
    Text(String),
    Kw(Kw),
    Eof,
}

impl Token {
    /// Convert a token back to the text form used in error messages.
    pub fn as_lexeme(&self) -> String {
        match self {
            Token::Hash => "#".into(),
            Token::Word(w) => w.clone(),
            Token::Text(t) => t.clone(),
            Token::Kw(k) => format!("{:?}", k),
            Token::Eof => "<EOF>".into(),
        }
    }
}

/// Convert a string to a keyword if it matches.
/// This is used by the lexer when detecting commands after `#` or `GIMMEH`.
pub fn map_kw(s: &str) -> Option<Kw> {
    match s.to_ascii_uppercase().as_str() {
        "HAI" => Some(Kw::Hai),
        "KTHXBYE" => Some(Kw::Kthxbye),
        "OBTW" => Some(Kw::OBTW),
        "TLDR" => Some(Kw::TLDR),
        "MAEK" => Some(Kw::Maek),
        "GIMMEH" => Some(Kw::Gimmeh),
        "OIC" => Some(Kw::OIC),
        "MKAY" => Some(Kw::Mkay),
        "HEAD" => Some(Kw::Head),
        "TITLE" => Some(Kw::Title),
        "PARAGRAF" => Some(Kw::Paragraf),
        "BOLD" => Some(Kw::Bold),
        "ITALICS" => Some(Kw::Italics),
        "LIST" => Some(Kw::List),
        "ITEM" => Some(Kw::Item),
        "NEWLINE" => Some(Kw::Newline),
        "SOUNDZ" => Some(Kw::Soundz),
        "VIDZ" => Some(Kw::Vidz),
        "I" => Some(Kw::I),
        "HAZ" => Some(Kw::Haz),
        "IT" => Some(Kw::It),
        "IZ" => Some(Kw::Iz),
        "LEMME" => Some(Kw::Lemme),
        "SEE" => Some(Kw::See),
        _ => None,
    }
}
