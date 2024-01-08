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

    pub fn next_strict(&mut self) -> Option<Token> {
        match self.inner.next() {
            Some(Ok(t)) => Some(t),
            Some(Err(_)) => None,
            None => Some(Token::Eof),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        loop {
            let t = self.next_strict()?;
            if t != Token::Newline {
                return Some(t);
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
