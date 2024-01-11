use std::borrow::BorrowMut;
use std::cell::RefMut;
use std::rc::Rc;

use lasso::{Rodeo, Spur};

use self::ast::Ast;
use self::error::SyntaxError;
use crate::errors::{DiagnosticBuilder, ErrorGuaranteed};
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

pub type ParseResult<'a, T, E = DiagnosticBuilder<'a>> = Result<T, E>;

#[non_exhaustive]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    session: &'a Session,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, session: &'a Session) -> Self {
        Self { lexer, session }
    }

    #[inline(always)]
    fn interner(&self) -> RefMut<'_, Rodeo<lasso::Spur, ahash::RandomState>> {
        self.session.interner.borrow_mut()
    }

    #[inline(always)]
    fn intern<T: AsRef<str>>(&mut self, string: T) -> Spur {
        self.interner().get_or_intern(string)
    }

    #[inline(always)]
    fn lexer_next_emit(&mut self, lexer: &mut Lexer) -> ParseResult<'a, Token> {
        match lexer.next() {
            Some(t) => Ok(t),
            None => Err(self
                .session
                .diag_ctx
                .create_error(SyntaxError::LexingError {
                    span: self.lexer.span(),
                })),
        }
    }

    #[inline(always)]
    fn lexer_next_strict_emit(&mut self, lexer: &mut Lexer) -> ParseResult<'a, Token> {
        match lexer.next_strict() {
            Some(t) => Ok(t),
            None => Err(self
                .session
                .diag_ctx
                .create_error(SyntaxError::LexingError {
                    span: self.lexer.span(),
                })),
        }
    }

    #[inline(always)]
    pub(crate) fn next(&mut self) -> ParseResult<'a, Token> {
        // self.lexer_next_emit(&mut self.lexer) <-- if only u could do this.....
        let next = self.lexer.next();
        dbg!(&next);
        if let Some(t) = next {
            return Ok(t);
        }

        Err(self
            .session
            .diag_ctx
            .create_error(SyntaxError::LexingError {
                span: self.lexer.span(),
            }))
    }

    #[inline(always)]
    pub(crate) fn next_strict(&mut self) -> ParseResult<'a, Token> {
        if let Some(t) = self.lexer.next_strict() {
            return Ok(t);
        }

        Err(self
            .session
            .diag_ctx
            .create_error(SyntaxError::LexingError {
                span: self.lexer.span(),
            }))
    }

    pub(crate) fn peek(&mut self) -> ParseResult<'a, Token> {
        let mut peek = self.lexer.clone();
        self.lexer_next_emit(&mut peek)
    }

    pub(crate) fn peek_strict(&mut self) -> ParseResult<'a, Token> {
        let mut peek = self.lexer.clone();
        self.lexer_next_strict_emit(&mut peek)
    }

    #[inline(always)]
    pub(crate) fn span(&self) -> CodeSpan {
        self.lexer.span()
    }

    pub(crate) fn peek_span(&mut self) -> ParseResult<'a, CodeSpan> {
        let mut peek = self.lexer.clone();
        self.lexer_next_emit(&mut peek)?;
        Ok(peek.span())
    }

    pub(crate) fn peek_span_strict(&mut self) -> ParseResult<'a, CodeSpan> {
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
        self.interner().get_or_intern(self.lexer.slice())
    }

    #[inline(always)]
    pub(crate) fn next_is(&mut self, tok: Token) -> ParseResult<'a, bool> {
        let peek = self.peek()?;
        Ok(peek == tok)
    }

    pub(crate) fn next_are(&mut self, toks: &[Token]) -> ParseResult<'a, bool> {
        let mut peek = self.lexer.clone();

        for tok in toks {
            if self.lexer_next_emit(&mut peek)? != *tok {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(crate) fn skip_tok(&mut self, skip: Token) -> ParseResult<'a, bool> {
        let next = self.next_is(skip)?;
        if next {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // pub(crate) fn skip_tok_recover(&mut self, skip: Token) -> bool {
    //     let next = self.next_is(skip).map_err(|e| e.emit());
    //     match next {
    //         Ok(true) => {
    //             if self.peek().map_err(|e| e.emit()).is_ok() {
    //                 self.next().map_err(|e| e.emit()).is_ok()
    //             }
    //         },
    //         Ok(false) => false,
    //         Err(_) => false,
    //     }
    // }

    pub(crate) fn expect_tok_named(&mut self, expect: Token, name: &str) -> ParseResult<'a, ()> {
        let next = self.next()?;
        if next != expect {
            return Err(self
                .session
                .diag_ctx
                .create_error(SyntaxError::UnexpectedToken {
                    found: next,
                    expected: name.to_string(),
                    span: self.span(),
                }));
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn expect_tok(&mut self, expect: Token) -> ParseResult<'a, ()> {
        self.expect_tok_named(expect, expect.name())
    }

    pub fn parse(&mut self) -> ParseResult<'a, Ast, ErrorGuaranteed> {
        let statements = self.parse_statements().map_err(|e| e.emit())?;

        // end parsing if we have errors that were not propogated from other functions
        if let Some(errors) = self.session.diag_ctx.abort_if_errors() {
            return Err(errors);
        }

        Ok(Ast { statements })
    }
}
