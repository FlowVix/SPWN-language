use super::ast::stmt::{Statements, StmtNode, StmtType};
use super::error::SyntaxError;
use super::{ParseResult, Parser};
use crate::errors::ErrorGuaranteed;
use crate::lexer::token::Token;

impl<'a> Parser<'a> {
    pub fn parse_block(&mut self) -> ParseResult<Statements> {
        self.expect_tok(Token::OpenBracket)?;
        let code = self.parse_statements()?;
        self.expect_tok(Token::ClosedBracket)?;

        Ok(code)
    }

    pub fn parse_statement(&mut self) -> ParseResult<StmtNode> {
        let start = self.peek_span()?;

        // let mut attrs = self.parse_outer_attributes()?;

        let is_arrow = self.skip_tok(Token::Arrow)?;

        let inner_start = self.peek_span()?;

        let typ = match self.peek()? {
            Token::If => {
                self.next()?;
                let mut branches = vec![];
                let mut else_branch = None;

                let cond = self.parse_expr()?;
                let code = self.parse_block()?;
                branches.push((cond, code));

                while self.skip_tok(Token::Else)? {
                    if self.skip_tok(Token::If)? {
                        let has_paren = self.skip_tok(Token::OpenParen)?;
                        let cond = self.parse_expr()?;
                        if has_paren {
                            self.expect_tok(Token::ClosedParen)?;
                        }
                        let code = self.parse_block()?;
                        branches.push((cond, code));
                    } else {
                        else_branch = Some(self.parse_block()?);
                        break;
                    }
                }

                StmtType::If {
                    branches,
                    else_branch,
                }
            },
            Token::While => {
                self.next()?;
                let cond = self.parse_expr()?;
                let code = self.parse_block()?;

                StmtType::While { cond, code }
            },
            Token::For => {
                self.next()?;
                let iter = self.parse_pattern()?;
                self.expect_tok(Token::In)?;
                let iterator = self.parse_expr()?;

                let code = self.parse_block()?;

                StmtType::For {
                    iter,
                    iterator,
                    code,
                }
            },
            Token::Try => {
                self.next()?;
                let try_code = self.parse_block()?;

                self.expect_tok(Token::Catch)?;

                let catch_pat = if !self.skip_tok(Token::OpenBracket)? {
                    let v = Some(self.parse_pattern()?);
                    v
                } else {
                    None
                };

                let catch_code = self.parse_block()?;

                StmtType::TryCatch {
                    try_code,
                    catch_code,
                    catch_pat,
                }
            },
            Token::Return => {
                self.next()?;
                if matches!(
                    self.peek_strict()?,
                    Token::Semicolon | Token::ClosedBracket | Token::Eof | Token::Newline
                ) {
                    StmtType::Return(None)
                } else {
                    let val = self.parse_expr()?;

                    StmtType::Return(Some(val))
                }
            },
            Token::Continue => {
                self.next()?;

                StmtType::Continue
            },
            Token::Break => {
                self.next()?;

                StmtType::Break
            },
            // t @ (Token::Private | Token::Type) => {
            //     let vis = if matches!(t, Token::Private) {
            //         self.next();
            //         Vis::Private
            //     } else {
            //         Vis::Public
            //     };

            //     self.next();
            //     self.expect_tok(Token::TypeIndicator)?;
            //     let name = self.slice()[1..].to_string();

            //     Statement::TypeDef(vis(self.intern_string(name)))
            // }
            // Token::Impl => {
            //     self.next();
            //     self.expect_tok(Token::TypeIndicator)?;
            //     let name_span = self.span();
            //     let name = self.slice()[1..].to_string();
            //     self.expect_tok(Token::OpenBracket)?;
            //     let items = self.parse_dictlike(true)?;

            //     Statement::Impl {
            //         name: self.intern_string(name).spanned(name_span),
            //         items,
            //     }
            // }
            // Token::Overload => {
            //     self.next();

            //     let tok = self.next();

            //     let op = if tok == Token::Unary {
            //         let tok = self.next();
            //         if let Some(op) = tok.to_unary_op() {
            //             Operator::Unary(op)
            //         } else {
            //             return Err(SyntaxError::UnexpectedToken {
            //                 found: tok,
            //                 expected: "unary operator".into(),
            //                 span: self.span(),
            //             });
            //         }
            //     } else if let Some(op) = tok.to_bin_op() {
            //         Operator::Bin(op)
            //     } else if let Some(op) = tok.to_assign_op() {
            //         Operator::Assign(op)
            //     } else if tok == Token::Assign {
            //         Operator::EqAssign
            //     } else {
            //         return Err(SyntaxError::UnexpectedToken {
            //             found: tok,
            //             expected: "binary operator".into(),
            //             span: self.span(),
            //         });
            //     };

            //     self.expect_tok(Token::OpenBracket)?;

            //     let mut macros = vec![];

            //     list_helper!(self, ClosedBracket {
            //         let vis = if self.skip_tok(Token::Private) {
            //             Vis::Private
            //         } else {
            //             Vis::Public
            //         };

            //         macros.push(vis(self.parse_expr(true)?));
            //     });

            //     Statement::Overload { op, macros }
            // }
            Token::Throw => {
                self.next()?;

                StmtType::Throw(self.parse_expr()?)
            },
            Token::Unsafe => {
                self.next()?;

                let stmts = self.parse_block()?;

                StmtType::Unsafe(stmts)
            },
            _ => {
                // let old_lexer = self.lexer.clone();

                // let count = self.session.diag_ctx.error_count();

                // let p = self.parse_pattern();
                // let new_count = self.session.diag_ctx.error_count();

                // match p {
                //     Ok(pat) if new_count == count => {
                //         let tok = self.peek();
                //         if tok == Token::Assign {
                //             self.next();
                //             let e = self.parse_expr()?;
                //             StmtType::Assign(pat, e)
                //         } else if let Some(op) = tok.to_assign_op() {
                //             self.next();
                //             let e = self.parse_expr()?;
                //             StmtType::AssignOp(pat, op, e)
                //         } else {
                //             self.lexer = old_lexer;
                //             let e = self.parse_expr()?;
                //             StmtType::Expr(e)
                //         }
                //     },
                //     v => {
                //         println!("{} {}", count, new_count);
                //         self.lexer = old_lexer;
                //         let e = self.parse_expr()?;
                //         if self.skip_tok(Token::Assign) {
                //             todo!()
                //             //return Err(ErrorGuaranteed);
                //         }
                //         StmtType::Expr(e)
                //     },
                // }
                let old_lexer = self.lexer.clone();

                match self.parse_pattern() {
                    Ok(pat) => {
                        let tok = self.peek()?;
                        if tok == Token::Assign {
                            self.next()?;
                            let e = self.parse_expr()?;
                            StmtType::Assign(pat, e)
                        } else if let Some(op) = tok.to_assign_op() {
                            self.next()?;
                            let e = self.parse_expr()?;
                            StmtType::AssignOp(pat, op, e)
                        } else {
                            self.lexer = old_lexer;
                            let e = self.parse_expr()?;
                            StmtType::Expr(e)
                        }
                    },
                    Err(pat_err) => {
                        self.lexer = old_lexer;
                        let e = self.parse_expr()?;
                        if self.skip_tok(Token::Assign)? {
                            return Err(pat_err);
                        }
                        StmtType::Expr(e)
                    },
                }
            },
        };

        let inner_span = inner_start.extended(self.span());

        if !matches!(self.peek()?, Token::ClosedBracket)
            && !matches!(
                self.peek_strict()?,
                Token::Semicolon | Token::Newline | Token::Eof
            )
        {
            let found = self.next()?;
            return Err(self
                .session
                .diag_ctx
                .emit_error(SyntaxError::UnexpectedToken {
                    found,
                    expected: "statement separator (`;` or newline)".to_string(),
                    span: self.span(),
                }));
        }
        self.skip_tok(Token::Semicolon)?;

        let typ = if is_arrow {
            StmtType::Arrow(StmtNode {
                typ: Box::new(typ),
                span: inner_span,
            })
        } else {
            typ
        };

        Ok(StmtNode {
            typ: Box::new(typ),
            span: start.extended(self.span()),
        })
    }

    pub fn parse_statements(&mut self) -> ParseResult<Statements> {
        let mut statements = vec![];

        while !matches!(self.peek()?, Token::Eof | Token::ClosedBracket) {
            // match self.parse_statement() {
            //     Err(err) => return Err(err.emit()),
            //     Ok(stmt) => statements.push(stmt),
            // }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(statements)
    }
}
