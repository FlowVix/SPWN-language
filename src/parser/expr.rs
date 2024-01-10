use super::ast::expr::{ExprNode, ExprType};
use super::error::SyntaxError;
use super::{operators, ParseResult, Parser};
use crate::lexer::token::Token;
use crate::list_helper;
use crate::parser::ast::expr::MacroBody;

impl<'a> Parser<'a> {
    pub(crate) fn parse_unit(&'a mut self) -> ParseResult<'a, ExprNode> {
        let t = self.next();
        let start = self.span();

        let unary;

        let typ = match t {
            Token::Int => ExprType::Int(self.slice().parse().unwrap()),
            Token::HexInt => ExprType::Int(i64::from_str_radix(&self.slice()[2..], 16).unwrap()),
            Token::OctalInt => ExprType::Int(i64::from_str_radix(&self.slice()[2..], 8).unwrap()),
            Token::BinaryInt => ExprType::Int(i64::from_str_radix(&self.slice()[2..], 2).unwrap()),
            Token::Float => ExprType::Float(self.slice().parse().unwrap()),
            Token::OpenParen => {
                let snapshot = self.lexer.clone();

                let after_close = {
                    let mut depth = 1;
                    while depth > 0 {
                        match self.next() {
                            Token::OpenParen => depth += 1,
                            Token::ClosedParen => depth -= 1,

                            Token::Eof => {
                                return Err(self.session.diag_ctx.create_error(
                                    SyntaxError::UnmatchedToken {
                                        for_tok: Token::OpenParen,
                                        not_found: Token::ClosedParen,
                                        span: start,
                                    },
                                ))
                            },
                            _ => {},
                        }
                    }
                    self.next()
                };
                self.lexer = snapshot;

                let is_macro = matches!(
                    after_close,
                    Token::FatArrow | Token::OpenBracket | Token::Arrow
                );

                if !is_macro {
                    let v = self.parse_expr()?;
                    self.expect_tok(Token::ClosedParen)?;
                    *v.typ
                } else {
                    let mut args = vec![];
                    list_helper!(self, ClosedParen {
                        args.push(self.parse_pattern()?);
                    });
                    let ret_pat = if self.skip_tok(Token::Arrow) {
                        Some(self.parse_pattern()?)
                    } else {
                        None
                    };
                    let (body, body_span) = if self.skip_tok(Token::FatArrow) {
                        let e = self.parse_expr()?;
                        let span = e.span;
                        (MacroBody::Lambda(e), span)
                    } else {
                        let start = self.peek_span();
                        (
                            MacroBody::Normal(self.parse_block()?),
                            start.extended(self.span()),
                        )
                    };
                    ExprType::Macro {
                        body,
                        body_span,
                        args,
                        ret_pat,
                    }
                }
            },
            Token::Ident => ExprType::Var(self.slice_interned()),

            Token::True => ExprType::Bool(true),
            Token::False => ExprType::Bool(false),

            Token::OpenSqBracket => {
                let mut v = vec![];
                list_helper!(self, ClosedSqBracket {
                    v.push(self.parse_expr()?);
                });
                ExprType::Array(v)
            },
            Token::Dbg => ExprType::Dbg(self.parse_expr()?),

            unary_op
                if {
                    unary = operators::unary_prec(unary_op);
                    unary.is_some()
                } =>
            {
                let unary_prec = unary.unwrap();
                let next_prec = operators::next_infix(unary_prec);
                let val = match next_prec {
                    Some(next_prec) => self.parse_op(next_prec)?,
                    None => self.parse_value()?,
                };

                ExprType::UnaryOp(unary_op.to_unary_op().unwrap(), val)
            },
            t => {
                return Err(self
                    .session
                    .diag_ctx
                    .create_error(SyntaxError::UnexpectedToken {
                        expected: "expression".into(),
                        found: t,
                        span: self.span(),
                    }))
            },
        };
        Ok(ExprNode {
            typ: Box::new(typ),
            span: start.extended(self.span()),
        })
    }

    pub fn parse_value(&mut self) -> ParseResult<'a, ExprNode> {
        let mut value = self.parse_unit()?;

        loop {
            let prev_span = value.span;

            let typ = match self.peek_strict() {
                Token::OpenParen => {
                    self.next();
                    let mut params = vec![];
                    list_helper!(self, ClosedParen {
                        params.push(self.parse_expr()?);
                    });
                    ExprType::Call {
                        base: value,
                        params,
                    }
                },
                _ => break,
            };
            value = ExprNode {
                typ: Box::new(typ),
                span: prev_span.extended(self.span()),
            }
        }

        Ok(value)
    }

    pub fn parse_expr(&mut self) -> ParseResult<'a, ExprNode> {
        self.parse_op(0)
    }

    pub fn parse_op(&mut self, prec: usize) -> ParseResult<'a, ExprNode> {
        let next_prec = operators::next_infix(prec);

        let mut left = match next_prec {
            Some(next_prec) => self.parse_op(next_prec)?,
            None => self.parse_value()?,
        };

        while operators::is_infix_prec(self.peek(), prec) {
            let op = self.next();

            let right = if operators::prec_type(prec) == operators::OpType::Left {
                match next_prec {
                    Some(next_prec) => self.parse_op(next_prec)?,
                    None => self.parse_value()?,
                }
            } else {
                self.parse_op(prec)?
            };
            let new_span = left.span.extended(right.span);
            left = ExprNode {
                typ: Box::new(ExprType::BinOp(left, op.to_bin_op().unwrap(), right)),
                span: new_span,
            }
        }

        Ok(left)
    }
}
