use std::str::CharIndices;

use lasso::{Rodeo, Spur};

use self::error::LexerError;
use self::tokens::Token;
use crate::source::CodeSpan;

pub mod error;
pub mod tokens;

type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    src: &'a str,
    chars: std::iter::Peekable<CharIndices<'a>>,
    span: CodeSpan,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: src.char_indices().peekable(),
            span: CodeSpan::ZEROSPAN,
            // pos: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next().map(|v| v.1)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|v| v.1)
    }

    #[inline]
    pub fn span(&self) -> CodeSpan {
        self.span
    }

    #[inline]
    pub fn slice(&self) -> &str {
        &self.src[self.span.start..self.span.end]
    }

    #[inline]
    pub fn slice_interned(&self, interner: &mut Rodeo) -> Spur {
        interner.get_or_intern(&self.src[self.span.start..self.span.end])
    }

    fn update_span(&mut self) {
        self.span.end = if let Some(&(t, _)) = self.chars.peek() {
            t
        } else {
            self.src.len()
        };
    }

    fn next_token(&mut self) -> LexerResult<Option<Token>> {
        while let Some(&(idx, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.next_char();
                continue;
            }
            self.span.start = idx;
            break;
        }

        macro_rules! numbers {
            () => {{
                let mut is_float = false;
                loop {
                    if self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                        self.next_char();
                        continue;
                    }
                    if self.peek_char() == Some('.') {
                        self.next_char();
                        is_float = true;
                        continue;
                    }
                    break;
                }

                Ok(Some(if is_float { Token::Float } else { Token::Int }))
            }};
        }
        macro_rules! ident_pattern {
            (Start) => {'A'..='Z' | 'a'..='z' | '_'};
            (Continue) => {'A'..='Z' | 'a'..='z' | '0'..='9' | '_'};
        }

        match self.next_char() {
            Some('+') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::PlusEq))
                },
                _ => Ok(Some(Token::Plus)),
            },
            Some('-') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::MinusEq))
                },
                Some('>') => {
                    self.next_char();
                    Ok(Some(Token::Arrow))
                },
                _ => Ok(Some(Token::Minus)),
            },
            Some('*') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::MultEq))
                },
                Some('*') => match self.peek_char() {
                    Some('=') => {
                        self.next_char();
                        Ok(Some(Token::PowEq))
                    },
                    _ => Ok(Some(Token::Pow)),
                },
                _ => Ok(Some(Token::Mult)),
            },
            Some('/') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::DivEq))
                },
                Some('/') => loop {
                    match self.chars.next() {
                        Some((idx, '\n')) => {
                            self.span.start = idx;
                            return self.next_token();
                        },
                        None => return Ok(None),
                        _ => {},
                    }
                },
                _ => Ok(Some(Token::Div)),
            },
            Some('%') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::ModEq))
                },
                _ => Ok(Some(Token::Mod)),
            },

            Some('(') => Ok(Some(Token::OpenParen)),
            Some(')') => Ok(Some(Token::ClosedParen)),
            Some('[') => Ok(Some(Token::OpenSqBracket)),
            Some(']') => Ok(Some(Token::ClosedSqBracket)),
            Some('{') => Ok(Some(Token::OpenBracket)),
            Some('}') => Ok(Some(Token::ClosedBracket)),
            Some(';') => Ok(Some(Token::Semicolon)),
            Some(':') => Ok(Some(Token::Colon)),
            Some('.') => Ok(Some(Token::Dot)),
            Some(',') => Ok(Some(Token::Comma)),

            Some('=') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::Eq))
                },
                Some('>') => {
                    self.next_char();
                    Ok(Some(Token::FatArrow))
                },
                _ => Ok(Some(Token::Assign)),
            },
            Some('!') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::NotEq))
                },
                _ => Ok(Some(Token::ExclMark)),
            },

            Some('>') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::Gte))
                },
                _ => Ok(Some(Token::Gt)),
            },
            Some('<') => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    Ok(Some(Token::Lte))
                },
                _ => Ok(Some(Token::Lt)),
            },
            Some('0') => match self.peek_char() {
                Some('x') => {
                    self.next_char();
                    if !matches!(self.peek_char(), Some('A'..='F' | 'a'..='f' | '0'..='9')) {
                        return Err(LexerError::InvalidHexLiteral);
                    }
                    self.next_char();
                    if matches!(
                        self.peek_char(),
                        Some('A'..='F' | 'a'..='f' | '0'..='9' | '_')
                    ) {
                        self.next_char();
                    }
                    Ok(Some(Token::HexInt))
                },
                Some('o') => {
                    self.next_char();
                    if !matches!(self.peek_char(), Some('0'..='7')) {
                        return Err(LexerError::InvalidOctalLiteral);
                    }
                    self.next_char();
                    if matches!(self.peek_char(), Some('0'..='7' | '_')) {
                        self.next_char();
                    }
                    Ok(Some(Token::OctalInt))
                },
                Some('b') => {
                    self.next_char();
                    if !matches!(self.peek_char(), Some('0'..='1')) {
                        return Err(LexerError::InvalidBinaryLiteral);
                    }
                    self.next_char();
                    if matches!(self.peek_char(), Some('0'..='1' | '_')) {
                        self.next_char();
                    }
                    // String::from_utf8(vec)
                    Ok(Some(Token::BinaryInt))
                },
                Some('s') => {
                    self.next_char();
                    if !matches!(self.peek_char(), Some('0'..='5')) {
                        return Err(LexerError::InvalidSeximalLiteral);
                    }
                    self.next_char();
                    if matches!(self.peek_char(), Some('0'..='5' | '_')) {
                        self.next_char();
                    }
                    Ok(Some(Token::SeximalInt))
                },
                Some('χ') => {
                    self.next_char();
                    if !matches!(self.peek_char(), Some('A'..='B' | 'a'..='b' | '0'..='9')) {
                        return Err(LexerError::InvalidDozenalLiteral);
                    }
                    self.next_char();
                    if matches!(
                        self.peek_char(),
                        Some('A'..='B' | 'a'..='b' | '0'..='9' | '_')
                    ) {
                        self.next_char();
                    }
                    Ok(Some(Token::DozenalInt))
                },
                Some('φ') => {
                    self.next_char();
                    if !matches!(self.peek_char(), Some('0'..='1')) {
                        return Err(LexerError::InvalidGoldenLiteral);
                    }
                    self.next_char();
                    if matches!(self.peek_char(), Some('0'..='1' | '_')) {
                        self.next_char();
                    }
                    Ok(Some(Token::GoldenFloat))
                },
                _ => numbers!(),
            },
            Some('0'..='9') => numbers!(),
            Some(ident_pattern!(Start)) => {
                while self
                    .peek_char()
                    .is_some_and(|v| matches!(v, ident_pattern!(Continue)))
                {
                    self.next_char();
                }
                self.update_span();

                Ok(Some(match self.slice() {
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "return" => Token::Return,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    "let" => Token::Let,
                    "dbg" => Token::Dbg,
                    "import" => Token::Import,
                    "r" => {
                        let hashes: u8 = match self.peek_char() {
                            Some('#') => {
                                self.next_char();
                                let mut hashes = 1;
                                while self.peek_char() == Some('#') {
                                    hashes += 1;
                                    self.next_char();
                                }
                                hashes
                            },
                            Some('\'' | '"') => 0,
                            _ => return Ok(Some(Token::Ident)),
                        };
                        let c = match self.next_char() {
                            Some(c @ ('\'' | '"')) => c,
                            _ => return Err(LexerError::InvalidCharacterForRawString),
                        };

                        loop {
                            match self.next_char() {
                                Some(t) if t == c => {
                                    let mut count = hashes;

                                    loop {
                                        if count == 0 {
                                            return Ok(Some(Token::RawString));
                                        }

                                        match self.next_char() {
                                            Some('#') => {
                                                count -= 1;
                                            },
                                            None => {
                                                return Err(LexerError::UnterminatedString);
                                            },
                                            _ => break,
                                        }
                                    }
                                },
                                None => return Err(LexerError::UnterminatedString),
                                _ => {},
                            }
                        }
                    },
                    _ => Token::Ident,
                }))
            },
            Some(c @ ('\'' | '"')) => {
                // println!("za");
                loop {
                    match self.next_char() {
                        Some('\\') => {
                            self.next_char();
                        },
                        Some(t) => {
                            if t == c {
                                break Ok(Some(Token::String));
                            }
                        },
                        _ => return Err(LexerError::UnterminatedString),
                    }
                }
            },
            Some(_) => Err(LexerError::UnknownCharacter),
            None => Ok(None),
        }
    }

    pub fn next(&mut self) -> Result<Option<Token>, LexerError> {
        self.span.start = self.span.end;
        let out = self.next_token();
        self.update_span();
        // println!("-> {:?} {:?}", out, self.span);
        out
    }

    pub fn next_or_eof(&mut self) -> Result<Token, LexerError> {
        self.next().map(|v| v.unwrap_or(Token::Eof))
    }
}
