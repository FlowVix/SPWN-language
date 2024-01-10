use ahash::AHashMap;

use super::ast::expr::{ExprNode, ExprType};
use super::ast::pattern::{PatternNode, PatternType};
use super::error::SyntaxError;
use super::{operators, ParseResult, Parser};
use crate::lexer::token::Token;
use crate::list_helper;
use crate::parser::ast::pattern::AssignPath;

impl<'a> Parser<'a> {
    fn parse_pattern_unit(&mut self) -> ParseResult<PatternNode> {
        let t = self.next();
        let start = self.span();

        macro_rules! dictlike_destructure {
            ($map:ident) => {
                let mut $map = AHashMap::new();
                list_helper!(self, ClosedBracket {
                    self.expect_tok(Token::Ident)?;

                    let k = self.slice_interned();
                    let start = self.span();

                    let p = if self.skip_tok(Token::Colon) {
                        Some(self.parse_pattern()?)
                    } else {
                        None
                    };
                    $map.insert(k, (p, start.extended(self.span())))
                });
            };
        }

        let typ = match t {
            Token::Any => PatternType::Any,

            #[allow(clippy::unnecessary_to_owned)]
            Token::Type => {
                let t = self.intern(self.slice()[1..].to_string());
                if self.skip_tok(Token::DoubleColon) {
                    dictlike_destructure!(map);
                    PatternType::InstanceDestructure(t, map)
                } else {
                    PatternType::Type(t)
                }
            },

            Token::Mut => {
                let prev = self.span();

                if self.skip_tok(Token::Slf) {
                    let span = prev.extended(self.span());
                    self.session
                        .diag_ctx
                        .emit_error(SyntaxError::MutSelf { span });

                    PatternType::Mut {
                        name: self.intern("self"),
                    }
                } else {
                    self.expect_tok(Token::Ident)?;
                    PatternType::Mut {
                        name: self.slice_interned(),
                    }
                }
            },
            Token::Ident => {
                let var = self.slice_interned();

                let mut path = vec![];

                loop {
                    match self.peek_strict() {
                        Token::OpenSqBracket => {
                            let index = self.parse_expr()?;
                            self.expect_tok(Token::ClosedSqBracket)?;

                            path.push(AssignPath::Index(index));
                        },
                        _ => match self.peek() {
                            Token::Dot => {
                                self.expect_tok(Token::Ident)?;
                                let member = self.slice_interned();
                                path.push(AssignPath::Member(member));
                            },
                            Token::DoubleColon => {
                                self.expect_tok(Token::Ident)?;
                                let member = self.slice_interned();
                                path.push(AssignPath::Associated(member));
                            },
                            _ => break,
                        },
                    }
                }

                PatternType::Path { var, path }
            },
            Token::Ampersand => match self.peek() {
                Token::Slf | Token::Ident => {

                    PatternType::Ref {
                        name: self.slice_interned(),
                    }
                },
                t => {
                    return Err(self
                        .session
                        .diag_ctx
                        .create_error(SyntaxError::UnexpectedToken {
                            expected: "`self` or identifier".into(),
                            found: t,
                            span: self.span(),
                        }));
                },
            },

            Token::Eq => PatternType::Eq(self.parse_expr()?),
            Token::Gt => PatternType::Gt(self.parse_expr()?),
            Token::GtE => PatternType::GtE(self.parse_expr()?),
            Token::Lt => PatternType::Lt(self.parse_expr()?),
            Token::LtE => PatternType::LtE(self.parse_expr()?),
            Token::NEq => PatternType::NEq(self.parse_expr()?),
            Token::In => PatternType::In(self.parse_expr()?),

            Token::OpenSqBracket => {
                let mut v = vec![];
                list_helper!(self, ClosedSqBracket {
                    v.push(self.parse_pattern()?);
                });
                PatternType::ArrayDestructure(v)
            },
            Token::OpenBracket => {
                dictlike_destructure!(map);
                PatternType::DictDestructure(map)
            },
            Token::OpenParen => {
                let v = self.parse_pattern()?;
                self.expect_tok(Token::ClosedParen)?;
                *v.typ
            },

            t => {
                return Err(self
                    .session
                    .diag_ctx
                    .create_error(SyntaxError::UnexpectedToken {
                        expected: "pattern".into(),
                        found: t,
                        span: self.span(),
                    }))
            },
        };
        Ok(PatternNode {
            typ: Box::new(typ),
            span: start.extended(self.span()),
        })
    }

    fn parse_pattern_value(&mut self) -> ParseResult<PatternNode> {
        let mut pat = self.parse_pattern_unit()?;

        loop {
            let prev_span = pat.span;

            let typ = match self.peek_strict() {
                Token::OpenSqBracket => {
                    if self.skip_tok(Token::ClosedSqBracket) {
                        PatternType::ArrayPattern(pat, None)
                    } else {
                        let inner = self.parse_pattern()?;
                        self.expect_tok(Token::ClosedSqBracket)?;
                        PatternType::ArrayPattern(pat, Some(inner))
                    }
                },
                Token::OpenBracket => {
                    self.expect_tok(Token::ClosedBracket)?;

                    PatternType::DictPattern(pat)
                },
                Token::QMark => {

                    PatternType::MaybeDestructure(Some(pat))
                },
                _ => break,
            };
            pat = PatternNode {
                typ: Box::new(typ),
                span: prev_span.extended(self.span()),
            }
        }

        Ok(pat)
    }

    fn parse_pattern_op(&mut self, prec: usize) -> ParseResult<PatternNode> {
        let next_prec = if prec == 2 { None } else { Some(prec + 1) };

        let mut left = match next_prec {
            Some(next_prec) => self.parse_pattern_op(next_prec)?,
            None => self.parse_pattern_value()?,
        };

        while matches!(
            (self.peek(), prec),
            (Token::Colon, 0) | (Token::Pipe, 1) | (Token::Ampersand, 2)
        ) {
            let peek = self.peek();
            let right = match next_prec {
                Some(next_prec) => self.parse_pattern_op(next_prec)?,
                None => self.parse_pattern_value()?,
            };
            let new_span = left.span.extended(right.span);

            left = PatternNode {
                span: new_span,
                typ: Box::new({
                    let p = match peek {
                        Token::Colon => PatternType::Both,
                        Token::Pipe => PatternType::Either,
                        Token::Ampersand => PatternType::Both,
                        _ => unreachable!(),
                    };
                    p(left, right)
                }),
            };
        }

        Ok(left)
    }

    pub fn parse_pattern(&mut self) -> ParseResult<PatternNode> {
        let mut pat = self.parse_pattern_op(0)?;

        loop {
            let start_span = pat.span;
            let typ = match self.peek_strict() {
                Token::If => {
                    let cond = self.parse_expr()?;
                    PatternType::IfGuard { pat, cond }
                },
                _ => {
                    break;
                },
            };
            pat = PatternNode {
                typ: Box::new(typ),
                span: start_span.extended(self.span()),
            };
        }

        Ok(pat)
    }
}
