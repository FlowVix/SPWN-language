use lasso::{Rodeo, Spur};

use self::ast::Ast;
use self::error::SyntaxError;
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::source::{CodeArea, CodeSpan, SpwnSource};

pub mod ast;
pub mod error;
pub mod expr;
pub mod operators;
pub mod pattern;
pub mod stmt;
pub mod util;

pub type ParseResult<T> = Result<T, SyntaxError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    src: &'static SpwnSource,
    interner: &'a mut Rodeo,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, src: &'static SpwnSource, interner: &'a mut Rodeo) -> Self {
        Self {
            lexer,
            src,
            interner,
        }
    }

    pub fn make_area(&self, span: CodeSpan) -> CodeArea {
        CodeArea {
            span,
            src: self.src,
        }
    }

    fn intern<T: AsRef<str>>(&mut self, string: T) -> Spur {
        self.interner.get_or_intern(string)
    }

    pub fn resolve(&self, s: &Spur) -> &str {
        self.interner.resolve(s)
    }

    pub fn next(&mut self) -> ParseResult<Token> {
        self.lexer.next(self.src)
    }

    pub fn next_strict(&mut self) -> ParseResult<Token> {
        self.lexer.next_strict(self.src)
    }

    pub fn peek(&self) -> ParseResult<Token> {
        let mut peek = self.lexer.clone();
        peek.next(self.src)
    }

    pub fn peek_strict(&self) -> ParseResult<Token> {
        let mut peek = self.lexer.clone();
        peek.next_strict(self.src)
    }

    pub fn span(&self) -> CodeSpan {
        self.lexer.span()
    }

    pub fn peek_span(&self) -> ParseResult<CodeSpan> {
        let mut peek = self.lexer.clone();
        peek.next(self.src)?;
        Ok(peek.span())
    }

    pub fn peek_span_strict(&self) -> ParseResult<CodeSpan> {
        let mut peek = self.lexer.clone();
        peek.next_strict(self.src)?;
        Ok(peek.span())
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn slice_interned(&mut self) -> Spur {
        self.interner.get_or_intern(self.lexer.slice())
    }

    pub fn next_is(&self, tok: Token) -> ParseResult<bool> {
        Ok(self.peek()? == tok)
    }

    pub fn next_are(&self, toks: &[Token]) -> ParseResult<bool> {
        let mut peek = self.lexer.clone();

        for tok in toks {
            if peek.next(self.src)? != *tok {
                return Ok(false);
            }
        }
        Ok(true)
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
        self.expect_tok_named(expect, expect.name())
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        Ok(Ast {
            statements: self.parse_statements()?,
        })
    }
}
