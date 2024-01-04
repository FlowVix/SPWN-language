use logos::Logos;

use self::token::Token;
use crate::parser::error::SyntaxError;
use crate::parser::ParseResult;
use crate::source::{CodeArea, CodeSpan, SpwnSource};

pub mod token;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            inner: Token::lexer(code),
        }
    }

    pub fn next_strict(&mut self, src: &'static SpwnSource) -> ParseResult<Token> {
        match self.inner.next() {
            Some(Ok(t)) => Ok(t),
            Some(Err(_)) => Err(SyntaxError::LexingError {
                area: CodeArea {
                    span: self.inner.span().into(),
                    src,
                },
            }),
            None => Ok(Token::Eof),
        }
    }

    pub fn next(&mut self, src: &'static SpwnSource) -> ParseResult<Token> {
        loop {
            let t = self.next_strict(src)?;
            if t != Token::Newline {
                return Ok(t);
            }
        }
    }

    pub fn span(&self) -> CodeSpan {
        self.inner.span().into()
    }

    pub fn slice(&self) -> &str {
        self.inner.slice()
    }
}
