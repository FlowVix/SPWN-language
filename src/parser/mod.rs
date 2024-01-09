use std::rc::Rc;

use lasso::{Rodeo, Spur};

use self::ast::Ast;
use self::error::SyntaxError;
use crate::errors::ErrorGuaranteed;
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::session::Session;
use crate::source::{CodeSpan, Source, SpwnSource};
use crate::util::interner::Interner;

pub mod ast;
pub mod error;
pub mod expr;
pub mod operators;
pub mod pattern;
pub mod stmt;
pub mod util;

pub type ParseResult<T> = Result<T, ErrorGuaranteed>;

#[non_exhaustive]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    session: &'a mut Session,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, session: &'a mut Session) -> Self {
        Self { lexer, session }
    }

    // #[inline(always)]
    // pub fn make_area(&self, span: CodeSpan) -> CodeArea {
    //     CodeArea {
    //         span,
    //         src: self.session.input,
    //     }
    // }

    // #[inline(always)]
    // fn src(&self) -> &'static SpwnSource {
    //     self.session.input
    // }

    #[inline(always)]
    fn intern<T: AsRef<str>>(&mut self, string: T) -> Spur {
        self.session.interner.get_or_intern(string)
    }

    #[inline(always)]
    pub fn resolve(&self, s: &Spur) -> &str {
        self.session.interner.resolve(s)
    }

    #[inline(always)]
    fn lexer_next_emit(&mut self, lexer: &mut Lexer) -> ParseResult<Token> {
        match lexer.next() {
            Some(t) => Ok(t),
            None => Err(self.session.diag_ctx.emit_error(SyntaxError::LexingError {
                span: self.lexer.span(),
            })),
        }
    }

    #[inline(always)]
    fn lexer_next_strict_emit(&mut self, lexer: &mut Lexer) -> ParseResult<Token> {
        match lexer.next_strict() {
            Some(t) => Ok(t),
            None => Err(self.session.diag_ctx.emit_error(SyntaxError::LexingError {
                span: self.lexer.span(),
            })),
        }
    }

    #[inline(always)]
    pub(crate) fn next(&mut self) -> ParseResult<Token> {
        // self.lexer_next_emit(&mut self.lexer) <-- if only u could do this.....
        if let Some(t) = self.lexer.next() {
            return Ok(t);
        }

        Err(self.session.diag_ctx.emit_error(SyntaxError::LexingError {
            span: self.lexer.span(),
        }))
    }

    #[inline(always)]
    pub(crate) fn next_strict(&mut self) -> ParseResult<Token> {
        if let Some(t) = self.lexer.next_strict() {
            return Ok(t);
        }

        Err(self.session.diag_ctx.emit_error(SyntaxError::LexingError {
            span: self.lexer.span(),
        }))
    }

    pub(crate) fn peek(&mut self) -> ParseResult<Token> {
        let mut peek = self.lexer.clone();
        self.lexer_next_emit(&mut peek)
    }

    pub(crate) fn peek_strict(&mut self) -> ParseResult<Token> {
        let mut peek = self.lexer.clone();
        self.lexer_next_strict_emit(&mut peek)
    }

    #[inline(always)]
    pub(crate) fn span(&self) -> CodeSpan {
        self.lexer.span()
    }

    pub(crate) fn peek_span(&mut self) -> ParseResult<CodeSpan> {
        let mut peek = self.lexer.clone();
        self.lexer_next_emit(&mut peek)?;
        Ok(peek.span())
    }

    pub(crate) fn peek_span_strict(&mut self) -> ParseResult<CodeSpan> {
        let mut peek = self.lexer.clone();
        self.lexer_next_strict_emit(&mut peek)?;
        Ok(peek.span())
    }

    #[inline(always)]
    pub(crate) fn slice(&self) -> &str {
        self.lexer.slice()
    }

    #[inline(always)]
    pub(crate) fn slice_interned(&mut self) -> Spur {
        self.session.interner.get_or_intern(self.lexer.slice())
    }

    #[inline(always)]
    pub(crate) fn next_is(&mut self, tok: Token) -> ParseResult<bool> {
        Ok(self.peek()? == tok)
    }

    pub(crate) fn next_are(&mut self, toks: &[Token]) -> ParseResult<bool> {
        let mut peek = self.lexer.clone();

        for tok in toks {
            if self.lexer_next_emit(&mut peek)? != *tok {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn skip_tok(&mut self, skip: Token) -> ParseResult<bool> {
        if self.next_is(skip)? {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn expect_tok_named(&mut self, expect: Token, name: &str) -> ParseResult<()> {
        let next = self.next()?;
        if next != expect {
            return Err(self
                .session
                .diag_ctx
                .emit_error(SyntaxError::UnexpectedToken {
                    found: next,
                    expected: name.to_string(),
                    span: self.span(),
                }));
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn expect_tok(&mut self, expect: Token) -> ParseResult<()> {
        self.expect_tok_named(expect, expect.name())
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        let statements = self.parse_statements()?;

        // end parsing if we have errors that were not propogated from other functions
        if let Some(errors) = self.session.diag_ctx.abort_if_errors() {
            return Err(errors);
        }

        Ok(Ast { statements })
    }
}
