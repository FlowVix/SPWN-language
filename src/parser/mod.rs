use std::rc::Rc;

use lasso::{Rodeo, Spur};

use self::error::SyntaxError;
use crate::lexer::error::LexerError;
use crate::lexer::tokens::Token;
use crate::lexer::Lexer;
use crate::source::{CodeArea, CodeSpan, SpwnSource};

pub mod ast;
pub mod error;
pub mod exprs;
pub mod operators;
pub mod patterns;
pub mod stmts;
pub mod util;

pub type ParseResult<T> = Result<T, SyntaxError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    src: &'a Rc<SpwnSource>,
    interner: &'a mut Rodeo,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, src: &'a Rc<SpwnSource>, interner: &'a mut Rodeo) -> Self {
        Self {
            lexer,
            src,
            interner,
        }
    }

    fn map_lexer_err(&self, e: LexerError) -> SyntaxError {
        SyntaxError::LexingError {
            err: e,
            area: self.make_area(self.span()),
        }
    }

    pub fn next(&mut self) -> ParseResult<Token> {
        let out = self
            .lexer
            .next_or_eof()
            .map_err(|e| self.map_lexer_err(e))?;

        if out == Token::Newline {
            self.next()
        } else {
            Ok(out)
        }
    }

    pub fn next_strict(&mut self) -> ParseResult<Token> {
        self.lexer.next_or_eof().map_err(|e| self.map_lexer_err(e))
    }

    pub fn span(&self) -> CodeSpan {
        self.lexer.span()
    }

    pub fn peek_span(&self) -> ParseResult<CodeSpan> {
        let mut peek = self.lexer.clone();
        while peek.next_or_eof().map_err(|e| self.map_lexer_err(e))? == Token::Newline {}
        Ok(peek.span())
    }

    // pub fn peek_span_or_newline(&self) -> CodeSpan {
    //     let mut peek = self.lexer.clone();
    //     peek.next_or_eof();
    //     peek.span().into()
    // }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn slice_interned(&mut self) -> Spur {
        self.interner.get_or_intern(self.lexer.slice())
    }

    pub fn peek(&self) -> ParseResult<Token> {
        let mut peek = self.lexer.clone();
        let mut out = peek.next_or_eof().map_err(|e| self.map_lexer_err(e))?;
        while out == Token::Newline {
            // should theoretically never be more than one, but having a loop just in case doesn't hurt
            out = peek.next_or_eof().map_err(|e| self.map_lexer_err(e))?;
        }
        Ok(out)
    }

    pub fn peek_strict(&self) -> ParseResult<Token> {
        let mut peek = self.lexer.clone();
        peek.next_or_eof().map_err(|e| self.map_lexer_err(e))
    }

    pub fn next_is(&self, tok: Token) -> ParseResult<bool> {
        Ok(self.peek()? == tok)
    }

    pub fn make_area(&self, span: CodeSpan) -> CodeArea {
        CodeArea {
            span,
            src: self.src.clone(),
        }
    }

    pub fn skip_tok(&mut self, skip: Token) -> ParseResult<bool> {
        if self.next_is(skip)? {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn expect_tok_named(&mut self, expect: Token, name: &str) -> ParseResult<()> {
        let next = self.next()?;
        if next != expect {
            return Err(SyntaxError::UnexpectedToken {
                found: next,
                expected: name.to_string(),
                area: self.make_area(self.span()),
            });
        }
        Ok(())
    }

    pub fn expect_tok(&mut self, expect: Token) -> Result<(), SyntaxError> {
        self.expect_tok_named(expect, expect.to_str())
    }

    pub fn next_are(&self, toks: &[Token]) -> ParseResult<bool> {
        let mut peek = self.lexer.clone();

        for tok in toks {
            let mut peeked = peek.next_or_eof().map_err(|e| self.map_lexer_err(e))?;
            while peeked == Token::Newline {
                peeked = peek.next_or_eof().map_err(|e| self.map_lexer_err(e))?;
            }

            if peeked != *tok {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn intern<T: AsRef<str>>(&mut self, string: T) -> Spur {
        self.interner.get_or_intern(string)
    }

    pub fn resolve(&self, s: &Spur) -> &str {
        self.interner.resolve(s)
    }
}
