// lexer.rs
// This file performs lexical analysis (tokenizing).
// It reads the raw input text character-by-character and produces Tokens.
// The parser uses these Tokens to build the AST.

use crate::error::Result;
use crate::token::{Kw, Token};

/// A minimal lexer trait (kept only to match the project spec)
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, _c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// Character-by-character lexer.
pub struct CharLexer {
    chars: Vec<char>, // full input as characters
    pos: usize,       // current index into chars
    pub line: usize,  // current line (for error reporting)
    pub col: usize,   // current column (for error reporting)

    // true if the next word after '#' should be treated as a keyword
    after_hash: bool,

    // tracks the last keyword, to determine if the next word
    // *must* be another keyword (e.g., after MAEK or GIMMEH)
    prev_kw: Option<Kw>,
}

impl CharLexer {
    /// Construct a new lexer from the input source text.
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 0,
            after_hash: false,
            prev_kw: None,
        }
    }

    #[inline]
    fn eof(&self) -> bool { self.pos >= self.chars.len() }

    #[inline]
    fn peek(&self) -> char {
        if self.eof() { '\0' } else { self.chars[self.pos] }
    }

    /// Move forward one character and return it.
    /// Updates line/column.
    #[inline]
    fn bump(&mut self) -> char {
        let c = self.peek();
        if !self.eof() {
            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
        c
    }

    /// Read characters while `pred` is true, return them as a string.
    fn take_while<F: Fn(char) -> bool>(&mut self, pred: F) -> String {
        let mut s = String::new();
        while !self.eof() && pred(self.peek()) {
            s.push(self.bump());
        }
        s
    }

    /// Identifiers and words: letters, numbers, underscore.
    fn is_word_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    /// Allowed punctuation characters treated as text.
    fn is_text_punct(c: char) -> bool {
        matches!(c, ',' | '.' | '"' | ':' | '?' | '!' | '%' | '/' )
    }

    /// Convert a string to a keyword if it matches.
    fn map_kw(upper: &str) -> Option<Kw> {
        use Kw::*;
        match upper {
            "HAI" => Some(Hai),
            "KTHXBYE" => Some(Kthxbye),
            "OBTW" => Some(OBTW),
            "TLDR" => Some(TLDR),
            "MAEK" => Some(Maek),
            "GIMMEH" => Some(Gimmeh),
            "HEAD" => Some(Head),
            "TITLE" => Some(Title),
            "PARAGRAF" => Some(Paragraf),
            "OIC" => Some(OIC),
            "BOLD" => Some(Bold),
            "ITALICS" => Some(Italics),
            "NEWLINE" => Some(Newline),
            "SOUNDZ" => Some(Soundz),
            "VIDZ" => Some(Vidz),
            "LIST" => Some(List),
            "ITEM" => Some(Item),
            "LEMME" => Some(Lemme),
            "SEE" => Some(See),
            "I" => Some(I),
            "HAZ" => Some(Haz),
            "IT" => Some(It),
            "IZ" => Some(Iz),
            "MKAY" => Some(Mkay),
            _ => None,
        }
    }

    /// Some keywords require the *next* word also be a keyword.
    fn prev_kw_expects_keyword(prev: Option<Kw>) -> bool {
        matches!(prev, Some(Kw::Maek) | Some(Kw::Gimmeh) | Some(Kw::Lemme) | Some(Kw::I) | Some(Kw::It))
    }

    /// Return the next token from the input.
    pub fn next_token(&mut self) -> Result<Token> {
        if self.eof() {
            return Ok(Token::Eof);
        }

        let c = self.peek();

        // '#' always starts an annotation tag.
        if c == '#' {
            self.bump();
            self.after_hash = true;
            self.prev_kw = None;
            return Ok(Token::Hash);
        }

        // Whitespace comes through as Text, the parser will ignore empty text.
        if c.is_whitespace() {
            let t = self.take_while(|ch| ch.is_whitespace());
            return Ok(Token::Text(t));
        }

        // Letters/numbers/underscore form a word.
        if Self::is_word_char(c) {
            let word = self.take_while(Self::is_word_char);
            let upper = word.to_ascii_uppercase();

            let keyword_ok = self.after_hash || Self::prev_kw_expects_keyword(self.prev_kw);

            if keyword_ok {
                if let Some(kw) = Self::map_kw(&upper) {
                    self.after_hash = false;
                    self.prev_kw = Some(kw);
                    return Ok(Token::Kw(kw));
                }
            }

            // Otherwise it's just a normal word.
            self.after_hash = false;
            self.prev_kw = None;
            return Ok(Token::Word(word));
        }

        // Punctuation allowed in text.
        if Self::is_text_punct(c) {
            let t = self.take_while(Self::is_text_punct);
            return Ok(Token::Text(t));
        }

        // Anything else is treated as a single text character.
        let ch = self.bump();
        Ok(Token::Text(ch.to_string()))
    }
}

// Small required trait implementation (not used in actual parsing).
impl LexicalAnalyzer for CharLexer {
    fn get_char(&mut self) -> char { self.bump() }
    fn add_char(&mut self, _c: char) { }
    fn lookup(&self, s: &str) -> bool {
        Self::map_kw(&s.to_ascii_uppercase()).is_some()
    }
}
