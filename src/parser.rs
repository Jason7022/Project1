use crate::ast::Node;
use crate::error::{LolError, Result};
use crate::lexer::CharLexer;
use crate::token::{Kw, Token};

/// Defines the parsing functions used to read LOL code and build an AST.
/// Each method handles one grammar rule.
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self) -> Result<()>;
    fn parse_head(&mut self) -> Result<()>;
    fn parse_title(&mut self) -> Result<()>;
    fn parse_comment(&mut self) -> Result<()>;
    fn parse_body(&mut self) -> Result<()>;
    fn parse_paragraph(&mut self) -> Result<()>;
    fn parse_inner_paragraph(&mut self) -> Result<()>;
    fn parse_inner_text(&mut self) -> Result<()>;
    fn parse_variable_define(&mut self) -> Result<()>;
    fn parse_variable_use(&mut self) -> Result<()>;
    fn parse_bold(&mut self) -> Result<()>;
    fn parse_italics(&mut self) -> Result<()>;
    fn parse_list(&mut self) -> Result<()>;
    fn parse_list_items(&mut self) -> Result<()>;
    fn parse_inner_list(&mut self) -> Result<()>;
    fn parse_audio(&mut self) -> Result<()>;
    fn parse_video(&mut self) -> Result<()>;
    fn parse_newline(&mut self) -> Result<()>;
    fn parse_text(&mut self) -> Result<()>;
}

/// The parser holds:
/// - current lexer
/// - current lookahead token
/// - the AST being constructed
/// - a stack to support nested structures like PARAGRAF and LIST
pub struct Parser<'a> {
    lexer: CharLexer,
    look: Token,
    pub ast: Vec<Node>,
    stack: Vec<Vec<Node>>,
    _src: &'a str,
}

impl<'a> Parser<'a> {
    /// Creates a new parser and reads the first token.
    pub fn new(input: &'a str) -> Result<Self> {
        let mut lx = CharLexer::new(input);
        let first = lx.next_token()?;
        Ok(Self {
            lexer: lx,
            look: first,
            ast: vec![],
            stack: vec![],
            _src: input,
        })
    }

    /// Moves to the next token.
    fn advance(&mut self) -> Result<()> {
        self.look = self.lexer.next_token()?;
        Ok(())
    }

    /// Ensures the current token is a specific keyword.
    fn expect_kw(&mut self, kw: Kw) -> Result<()> {
        if let Token::Kw(k) = &self.look {
            if *k == kw {
                self.advance()?;
                return Ok(());
            }
        }
        Err(LolError::Syntax {
            expected: format!("{:?}", kw),
            found: self.look.as_lexeme(),
        })
    }

    /// Ensures the current token is a '#'.
    fn expect_hash(&mut self) -> Result<()> {
        if matches!(self.look, Token::Hash) {
            self.advance()?;
            Ok(())
        } else {
            Err(LolError::Syntax {
                expected: "#".into(),
                found: self.look.as_lexeme(),
            })
        }
    }

    /// Adds a node either to the current nested block or to the root AST.
    fn push_node(&mut self, n: Node) {
        if let Some(top) = self.stack.last_mut() {
            top.push(n);
        } else {
            self.ast.push(n);
        }
    }

    /// Skips whitespace-only text tokens.
    fn skip_ws(&mut self) -> Result<()> {
        while let Token::Text(t) = &self.look {
            if t.trim().is_empty() {
                self.advance()?;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Reads text until another control symbol (`#`) appears.
    fn read_text_until_hash(&mut self) -> Result<String> {
        let mut out = String::new();
        loop {
            match &self.look {
                Token::Text(t) => { out.push_str(t); self.advance()?; }
                Token::Word(w) => { out.push_str(w); self.advance()?; }
                Token::Kw(_) | Token::Hash | Token::Eof => break,
            }
        }
        Ok(out.trim().to_string())
    }
}

impl<'a> SyntaxAnalyzer for Parser<'a> {

    /// Parses the whole LOL program.
    /// Must start with #HAI and end with #KTHXBYE.
    fn parse_lolcode(&mut self) -> Result<()> {
        self.expect_hash()?;
        self.expect_kw(Kw::Hai)?;

        // Start root block
        self.stack.push(vec![]);
        self.skip_ws()?;

        loop {
            self.skip_ws()?;

            match self.look {
                Token::Hash => {
                    self.advance()?;
                    self.skip_ws()?;

                    match &self.look {
                        Token::Kw(Kw::Kthxbye) => { self.advance()?; break; }
                        Token::Kw(Kw::OBTW) => self.parse_comment()?,
                        Token::Kw(Kw::Maek) => {
                            self.advance()?;
                            self.skip_ws()?;
                            match &self.look {
                                Token::Kw(Kw::Head)     => self.parse_head()?,
                                Token::Kw(Kw::Paragraf) => self.parse_paragraph()?,
                                Token::Kw(Kw::List)     => self.parse_list()?,
                                _ => return Err(LolError::Syntax {
                                    expected: "HEAD/PARAGRAF/LIST".into(),
                                    found: self.look.as_lexeme(),
                                })
                            }
                        }
                        Token::Kw(Kw::Gimmeh) => self.parse_body()?,
                        Token::Kw(Kw::Lemme) => self.parse_variable_use()?,
                        Token::Kw(Kw::I)     => self.parse_variable_define()?,

                        // If someone writes HEAD without MAEK first
                        Token::Kw(Kw::Head) => {
                            return Err(LolError::Syntax {
                                expected: "Use #MAEK HEAD ... #OIC".into(),
                                found: "HEAD".into(),
                            });
                        }

                        _ => return Err(LolError::Syntax {
                            expected: "valid top-level annotation".into(),
                            found: self.look.as_lexeme(),
                        })
                    }
                }

                // Allow text at top-level (HTML paragraph-like behavior)
                Token::Text(_) | Token::Word(_) => self.parse_text()?,

                Token::Eof => return Err(LolError::Syntax {
                    expected: "#KTHXBYE".into(),
                    found: "<EOF>".into(),
                }),

                _ => {}
            }
        }

        // Finalize AST
        if let Some(children) = self.stack.pop() {
            self.ast = children;
        }
        Ok(())
    }

    /// Parses a HEAD block.
    fn parse_head(&mut self) -> Result<()> {
        self.expect_kw(Kw::Head)?;
        self.stack.push(vec![]);

        loop {
            self.skip_ws()?;
            match self.look {
                Token::Hash => {
                    self.advance()?;
                    self.skip_ws()?;
                    match &self.look {
                        Token::Kw(Kw::Gimmeh) => { self.advance()?; self.skip_ws()?; self.parse_title()?; }
                        Token::Kw(Kw::OBTW) => self.parse_comment()?,
                        Token::Kw(Kw::OIC) => { self.advance()?; break; }
                        _ => return Err(LolError::Syntax {
                            expected: "GIMMEH TITLE or OBTW or OIC".into(),
                            found: self.look.as_lexeme(),
                        })
                    }
                }
                Token::Eof => return Err(LolError::Syntax {
                    expected: "#OIC".into(),
                    found: "<EOF>".into(),
                }),
                _ => { self.advance()?; }
            }
        }

        let kids = self.stack.pop().unwrap();
        self.push_node(Node::Head(kids));
        Ok(())
    }

    fn parse_title(&mut self) -> Result<()> {
        self.expect_kw(Kw::Title)?;
        let t = self.read_text_until_hash()?;
        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::Title(t));
        Ok(())
    }

    /// Reads OBTW ... TLDR comments.
    fn parse_comment(&mut self) -> Result<()> {
        self.expect_kw(Kw::OBTW)?;
        let mut text = String::new();
        loop {
            match &self.look {
                Token::Hash => { self.advance()?; self.skip_ws()?; self.expect_kw(Kw::TLDR)?; break; }
                Token::Text(t) => { text.push_str(t); self.advance()?; }
                Token::Word(w) => { text.push_str(w); self.advance()?; }
                Token::Eof => return Err(LolError::Syntax { expected: "#TLDR".into(), found: "<EOF>".into() }),
                _ => { self.advance()?; }
            }
        }
        self.push_node(Node::Comment(text.trim().to_string()));
        Ok(())
    }

    /// Parses a PARAGRAF block.
    fn parse_paragraph(&mut self) -> Result<()> {
        self.expect_kw(Kw::Paragraf)?;
        self.stack.push(vec![]);

        loop {
            self.skip_ws()?;
            match self.look {
                Token::Hash => {
                    self.advance()?;
                    self.skip_ws()?;
                    match &self.look {
                        Token::Kw(Kw::Gimmeh) => {
                            self.advance()?; self.skip_ws()?;
                            match &self.look {
                                Token::Kw(Kw::Bold)    => self.parse_bold()?,
                                Token::Kw(Kw::Italics) => self.parse_italics()?,
                                Token::Kw(Kw::Newline) => self.parse_newline()?,
                                Token::Kw(Kw::Soundz)  => self.parse_audio()?,
                                Token::Kw(Kw::Vidz)    => self.parse_video()?,
                                _ => return Err(LolError::Syntax { expected: "BOLD/ITALICS/NEWLINE/SOUNDZ/VIDZ".into(), found: self.look.as_lexeme() })
                            }
                        }
                        Token::Kw(Kw::Lemme) => self.parse_variable_use()?,
                        Token::Kw(Kw::I)     => self.parse_variable_define()?,
                        Token::Kw(Kw::OBTW)  => self.parse_comment()?,
                        Token::Kw(Kw::OIC)   => { self.advance()?; break; }
                        _ => return Err(LolError::Syntax { expected: "GIMMEH/LEMME/I/OBTW/OIC".into(), found: self.look.as_lexeme() })
                    }
                }
                Token::Text(_) | Token::Word(_) => self.parse_text()?,
                Token::Eof => return Err(LolError::Syntax { expected: "#OIC".into(), found: "<EOF>".into() }),
                _ => return Err(LolError::Syntax { expected: "content in PARAGRAF".into(), found: self.look.as_lexeme() }),
            }
        }

        let inner = self.stack.pop().unwrap();
        self.push_node(Node::Paragraph(inner));
        Ok(())
    }

    fn parse_inner_paragraph(&mut self) -> Result<()> { Ok(()) }
    fn parse_inner_text(&mut self) -> Result<()> { self.parse_text() }

    /// Variable definition:  I HAZ var IT IZ value #MKAY
    fn parse_variable_define(&mut self) -> Result<()> {
        self.expect_kw(Kw::I)?;
        self.expect_kw(Kw::Haz)?;

        let name = match &self.look {
            Token::Word(w) => { let s = w.clone(); self.advance()?; s }
            Token::Text(t) => { let s = t.split_whitespace().next().unwrap_or("").to_string(); self.advance()?; s }
            _ => return Err(LolError::Syntax { expected: "variable name".into(), found: self.look.as_lexeme() })
        };

        self.expect_kw(Kw::It)?;
        self.expect_kw(Kw::Iz)?;

        let value = self.read_text_until_hash()?;

        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;

        self.push_node(Node::VarDef { name, value });
        Ok(())
    }

    /// Variable use:  LEMME SEE var #MKAY
    fn parse_variable_use(&mut self) -> Result<()> {
        self.expect_kw(Kw::Lemme)?;
        self.expect_kw(Kw::See)?;

        let name = match &self.look {
            Token::Word(w) => { let s = w.clone(); self.advance()?; s }
            Token::Text(t) => { let s = t.split_whitespace().next().unwrap_or("").to_string(); self.advance()?; s }
            _ => return Err(LolError::Syntax { expected: "variable name".into(), found: self.look.as_lexeme() })
        };

        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::VarUse { name });
        Ok(())
    }

    /// Handles #GIMMEH BOLD / ITALICS / NEWLINE etc.
    fn parse_body(&mut self) -> Result<()> {
        self.expect_kw(Kw::Gimmeh)?;
        self.skip_ws()?;
        match self.look.clone() {
            Token::Kw(Kw::Bold)    => self.parse_bold(),
            Token::Kw(Kw::Italics) => self.parse_italics(),
            Token::Kw(Kw::Newline) => self.parse_newline(),
            Token::Kw(Kw::Soundz)  => self.parse_audio(),
            Token::Kw(Kw::Vidz)    => self.parse_video(),
            Token::Kw(Kw::Item)    => self.parse_list_items(),
            Token::Kw(Kw::Title)   => self.parse_title(),
            other => Err(LolError::Syntax { expected: "BOLD/ITALICS/NEWLINE/SOUNDZ/VIDZ/ITEM/TITLE".into(), found: other.as_lexeme() }),
        }
    }

    fn parse_bold(&mut self) -> Result<()> {
        self.expect_kw(Kw::Bold)?;
        let t = self.read_text_until_hash()?;
        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::Bold(t));
        Ok(())
    }

    fn parse_italics(&mut self) -> Result<()> {
        self.expect_kw(Kw::Italics)?;
        let t = self.read_text_until_hash()?;
        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::Italics(t));
        Ok(())
    }

    fn parse_newline(&mut self) -> Result<()> {
        self.expect_kw(Kw::Newline)?;
        self.push_node(Node::Newline);
        Ok(())
    }

    /// LIST block
    fn parse_list(&mut self) -> Result<()> {
        self.expect_kw(Kw::List)?;
        self.stack.push(vec![]);

        loop {
            self.skip_ws()?;
            match self.look {
                Token::Hash => {
                    self.advance()?;
                    self.skip_ws()?;
                    match &self.look {
                        Token::Kw(Kw::Gimmeh) => self.parse_list_items()?,
                        Token::Kw(Kw::OBTW)   => self.parse_comment()?,
                        Token::Kw(Kw::OIC)    => { self.advance()?; break; }
                        _ => return Err(LolError::Syntax { expected: "GIMMEH ITEM or OBTW or OIC".into(), found: self.look.as_lexeme() })
                    }
                }
                Token::Eof => return Err(LolError::Syntax { expected: "#OIC for LIST".into(), found: "<EOF>".into() }),
                _ => return Err(LolError::Syntax { expected: "# in LIST".into(), found: self.look.as_lexeme() }),
            }
        }

        let items = self.stack.pop().unwrap();
        self.push_node(Node::List(items));
        Ok(())
    }

    fn parse_list_items(&mut self) -> Result<()> {
        self.expect_kw(Kw::Item)?;
        let t = self.read_text_until_hash()?;
        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::ListItem(vec![Node::Text(t)]));
        Ok(())
    }

    fn parse_inner_list(&mut self) -> Result<()> { Ok(()) }

    fn parse_audio(&mut self) -> Result<()> {
        self.expect_kw(Kw::Soundz)?;
        let a = self.read_text_until_hash()?;
        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::Audio(a));
        Ok(())
    }

    fn parse_video(&mut self) -> Result<()> {
        self.expect_kw(Kw::Vidz)?;
        let a = self.read_text_until_hash()?;
        self.expect_hash()?;
        self.expect_kw(Kw::Mkay)?;
        self.push_node(Node::Video(a));
        Ok(())
    }

    /// Reads plain text tokens.
    fn parse_text(&mut self) -> Result<()> {
        let mut s = String::new();
        loop {
            match &self.look {
                Token::Text(t) => { s.push_str(t); self.advance()?; }
                Token::Word(w) => { s.push_str(w); self.advance()?; }
                _ => break,
            }
        }
        if !s.trim().is_empty() {
            self.push_node(Node::Text(s));
        }
        Ok(())
    }
}
