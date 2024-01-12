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

    // #[inline(always)]
    // fn lexer_next_emit(&mut self, lexer: &mut Lexer) -> ParseResult<'a, Token> {
    //     match lexer.next() {
    //         Some(t) => Ok(t),
    //         None => Err(self
    //             .session
    //             .diag_ctx
    //             .create_error(SyntaxError::LexingError {
    //                 span: self.lexer.span(),
    //             })),
    //     }
    // }

    // #[inline(always)]
    // fn lexer_next_strict_emit(&mut self, lexer: &mut Lexer) -> ParseResult<'a, Token> {
    //     match lexer.next_strict() {
    //         Some(t) => Ok(t),
    //         None => Err(self
    //             .session
    //             .diag_ctx
    //             .create_error(SyntaxError::LexingError {
    //                 span: self.lexer.span(),
    //             })),
    //     }
    // }

    #[inline(always)]
    pub(crate) fn next(&mut self) -> Token {
        // self.lexer_next_emit(&mut self.lexer) <-- if only u could do this.....
        self.lexer.next()
    }

    #[inline(always)]
    pub(crate) fn next_strict(&mut self) -> Token {
        self.lexer.next_strict()
    }

    pub(crate) fn peek(&mut self) -> Token {
        let mut peek = self.lexer.clone();
        peek.next()
    }

    pub(crate) fn peek_strict(&mut self) -> Token {
        let mut peek = self.lexer.clone();
        peek.next_strict()
    }

    #[inline(always)]
    pub(crate) fn span(&self) -> CodeSpan {
        self.lexer.span()
    }

    pub(crate) fn peek_span(&mut self) -> CodeSpan {
        let mut peek = self.lexer.clone();
        peek.next();
        peek.span()
    }

    pub(crate) fn peek_span_strict(&mut self) -> CodeSpan {
        let mut peek = self.lexer.clone();
        peek.next_strict();
        peek.span()
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
    pub(crate) fn next_is(&mut self, tok: Token) -> bool {
        let peek = self.peek();
        peek == tok
    }

    pub(crate) fn next_are(&mut self, toks: &[Token]) -> bool {
        let mut peek = self.lexer.clone();

        for tok in toks {
            if peek.next() != *tok {
                return false;
            }
        }
        true
    }

    pub(crate) fn skip_tok(&mut self, skip: Token) -> bool {
        let next = self.next_is(skip);
        if next {
            self.next();
            true
        } else {
            false
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
        let next = self.next();
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

    // bool represents if it recovered or not
    #[inline(always)]
    pub(crate) fn expect_tok_recover(&mut self, expect: Token) -> bool {
        self.expect_tok_named(expect, expect.name()).map_or_else(
            |e| {
                e.emit();
                true
            },
            |_| false,
        )
    }

    pub fn parse(&mut self) -> ParseResult<'a, Ast, ErrorGuaranteed> {
        let statements = self.parse_statements();

        // end parsing if we have errors that were not propogated from other functions
        if let Some(errors) = self.session.diag_ctx.abort_if_errors() {
            return Err(errors);
        }

        Ok(Ast { statements })
    }
}
