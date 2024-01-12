use logos::Logos;

use self::token::Token;
use crate::parser::error::SyntaxError;
use crate::parser::ParseResult;
use crate::source::{CodeSpan, SpwnSource};

pub mod token;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
    //current_pos: usize,
    //start_offset: usize,
    source_id: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str, source_id: usize) -> Self {
        Self {
            inner: Token::lexer(code),
            source_id,
            //start_offset,
            //current_pos: start_offset,
        }
    }

    pub fn next_strict(&mut self) -> Token {
        match self.inner.next() {
            Some(Ok(t)) => t,
            Some(Err(_)) => Token::Unknown,
            None => Token::Eof,
        }
    }

    pub fn next(&mut self) -> Token {
        loop {
            let t = self.next_strict();
            if t != Token::Newline {
                return t;
            }
        }
    }

    pub fn span(&self) -> CodeSpan {
        let span = self.inner.span();
        // let token_len = span.end - span.start;
        // CodeSpan {
        //     start: self.current_pos,
        //     end: self.current_pos + token_len,
        // }
        CodeSpan {
            start: span.start,
            end: span.end,
            source_id: self.source_id,
        }
    }

    pub fn slice(&self) -> &str {
        self.inner.slice()
    }
}
