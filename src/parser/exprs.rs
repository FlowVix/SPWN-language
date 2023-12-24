use ahash::AHashMap;

use super::ast::expr::{ExprNode, ExprType};
use super::error::SyntaxError;
use super::operators::{self, unary_prec};
use super::{ParseResult, Parser};
use crate::gd::ids::IDClass;
use crate::lexer::tokens::Token;
use crate::list_helper;
use crate::parser::ast::expr::{MacroArg, MacroCode, MatchBranch, MatchBranchCode};
use crate::parser::ast::pattern::{PatternNode, PatternType};

impl Parser<'_> {
    pub fn parse_unit(&mut self, allow_macros: bool) -> ParseResult<ExprNode> {
        //let attrs = self.parse_outer_attributes()?;

        let peek = self.peek()?;
        let start = self.peek_span()?;

        let unary;

        let expr = 'out_expr: {
            match peek {
                Token::Int => {
                    self.next()?;
                    ExprType::Int(self.parse_int(self.slice(), 10)?)
                },
                Token::HexInt => {
                    self.next()?;
                    ExprType::Int(self.parse_int(&self.slice()[2..], 16)?)
                },
                Token::OctalInt => {
                    self.next()?;
                    ExprType::Int(self.parse_int(&self.slice()[2..], 8)?)
                },
                Token::BinaryInt => {
                    self.next()?;
                    ExprType::Int(self.parse_int(&self.slice()[2..], 2)?)
                },
                Token::SeximalInt => {
                    self.next()?;
                    ExprType::Int(self.parse_int(&self.slice()[2..], 6)?)
                },
                Token::DozenalInt => {
                    self.next()?;
                    ExprType::Int(self.parse_int(&self.slice()[3..], 12)?)
                },
                Token::GoldenFloat => {
                    self.next()?;
                    ExprType::Float(self.parse_golden_float(&self.slice()[3..]))
                },
                Token::Float => {
                    self.next()?;
                    ExprType::Float(self.slice().replace('_', "").parse::<f64>().unwrap())
                },
                // Token::String => {
                //     let t = self.next()?;
                //     ExprType::String(self.parse_string(t)?)
                // },
                // Token::StringFlags => {
                //     let t = self.next()?;
                //     ExprType::String(self.parse_string(t)?)
                // },
                // Token::RawString => {
                //     let t = self.next()?;
                //     ExprType::String(self.parse_string(t)?)
                // },
                Token::ArbitraryGroupID => {
                    self.next()?;
                    ExprType::Id {
                        class: IDClass::Group,
                        value: None,
                    }
                },
                Token::ArbitraryItemID => {
                    self.next()?;
                    ExprType::Id {
                        class: IDClass::Item,
                        value: None,
                    }
                },
                Token::ArbitraryChannelID => {
                    self.next()?;
                    ExprType::Id {
                        class: IDClass::Channel,
                        value: None,
                    }
                },
                Token::ArbitraryBlockID => {
                    self.next()?;
                    ExprType::Id {
                        class: IDClass::Block,
                        value: None,
                    }
                },
                Token::GroupID => {
                    self.next()?;
                    self.parse_id(self.slice(), IDClass::Group)
                },
                Token::ItemID => {
                    self.next()?;
                    self.parse_id(self.slice(), IDClass::Item)
                },
                Token::ChannelID => {
                    self.next()?;
                    self.parse_id(self.slice(), IDClass::Channel)
                },
                Token::BlockID => {
                    self.next()?;
                    self.parse_id(self.slice(), IDClass::Block)
                },
                Token::Dollar => {
                    self.next()?;

                    ExprType::Builtins
                },
                Token::True => {
                    self.next()?;
                    ExprType::Bool(true)
                },
                Token::False => {
                    self.next()?;
                    ExprType::Bool(false)
                },
                Token::Epsilon => {
                    self.next()?;
                    ExprType::Epsilon
                },
                Token::TypeIndicator => {
                    self.next()?;
                    let name = self.slice()[1..].to_string();
                    ExprType::Type(self.intern(name))
                },
                Token::OpenSqBracket => {
                    self.next()?;

                    let mut elems = vec![];

                    list_helper!(self, ClosedSqBracket {
                        elems.push(self.parse_expr(true)?);
                    });

                    ExprType::Array(elems)
                },
                // typ @ (Token::Obj | Token::Trigger) => {
                //     self.next()?;

                //     self.expect_tok(Token::OpenBracket)?;

                //     let mut items: Vec<(Spanned<ObjKeyType>, ExprNode)> = vec![];

                //     list_helper!(self, ClosedBracket {
                //         let key = match self.next()? {
                //             Token::Int => ObjKeyType::Num(self.parse_int(self.slice(), 10)? as u8),
                //             Token::Ident => ObjKeyType::Name(*OBJECT_KEYS.get(self.slice()).ok_or(SyntaxError::UnknownObjectKey { area: self.make_area(self.span()) })?),
                //             other => {
                //                 return Err(SyntaxError::UnexpectedToken {
                //                     expected: "key".into(),
                //                     found: other,
                //                     area: self.make_area(self.span()),
                //                 })
                //             }
                //         };

                //         let key_span = self.span();
                //         self.expect_tok(Token::Colon)?;
                //         items.push((key.spanned(key_span), self.parse_expr(true)?));
                //     });

                //     ExprType::Obj(
                //         match typ {
                //             Token::Obj => ObjectType::Object,
                //             Token::Trigger => ObjectType::Trigger,
                //             _ => unreachable!(),
                //         },
                //         items,
                //     )
                // },
                Token::OpenBracket => {
                    todo!()
                    // self.next()?;

                    // ExprType::Dict(self.parse_dictlike(false)?)
                },
                Token::QMark => {
                    self.next()?;

                    ExprType::Maybe(None)
                },
                Token::TrigFnBracket => {
                    self.next()?;

                    let code = self.parse_statements()?;
                    self.expect_tok(Token::ClosedBracket)?;

                    ExprType::TriggerFunc { code }
                },
                Token::Import => {
                    todo!()
                    // self.next()?;

                    // let import_type = self.parse_import()?;

                    // ExprType::Import(import_type)
                },
                Token::Extract => {
                    todo!()
                    // self.next()?;

                    // let map = match self.peek()? {
                    //     Token::Import => {
                    //         self.next()?;

                    //         None
                    //     },
                    //     Token::OpenBracket => {
                    //         self.next()?;

                    //         let start_span = self.span();

                    //         let mut map = AHashMap::new();

                    //         list_helper!(self, ClosedBracket {
                    //             match self.next()? {
                    //                 Token::Ident => {
                    //                     let key = self.intern(self.slice());

                    //                     let key_span = self.span();

                    //                     let elem = if self.next_is(Token::Colon)? {
                    //                         self.next()?;
                    //                         self.parse_pattern()?
                    //                     } else {
                    //                         PatternNode {
                    //                             pat: Box::new(PatternType::Path {
                    //                                 var: key,
                    //                                 path: vec![],
                    //                             }),
                    //                             span: key_span,
                    //                         }
                    //                     };

                    //                     map.insert( ModuleDestructureKey::Ident(key).spanned(key_span), Some(elem));
                    //                 },
                    //                 t @ (Token::Private | Token::TypeIndicator) => {
                    //                     let vis = match t {
                    //                         Token::Private => {
                    //                             self.expect_tok(Token::TypeIndicator)?;
                    //                             Vis::Private
                    //                         },
                    //                         Token::TypeIndicator => {
                    //                             Vis::Public
                    //                         }
                    //                         _ => unreachable!(),
                    //                     };

                    //                     let key = ModuleDestructureKey::Type(vis(self.intern(&self.slice()[1..])));
                    //                     let key_span = self.span();

                    //                     map.insert(key.spanned(key_span), None);
                    //                 }
                    //                 other => {
                    //                     return Err(SyntaxError::UnexpectedToken {
                    //                         expected: "identifier or type indicator".into(),
                    //                         found: other,
                    //                         area: self.make_area(self.span()),
                    //                     })
                    //                 }
                    //             };
                    //         });

                    //         let map = Some(map.spanned(start_span.extended(self.span())));

                    //         self.expect_tok(Token::Import)?;

                    //         map
                    //     },
                    //     other => {
                    //         return Err(SyntaxError::UnexpectedToken {
                    //             expected: "`import` or module destructure pattern".into(),
                    //             found: other,
                    //             area: self.make_area(self.span()),
                    //         })
                    //     },
                    // };

                    // let import_type = self.parse_import()?;

                    // ExprType::ExtractImport {
                    //     import: import_type,
                    //     destructure: map,
                    // }
                },
                Token::Match => {
                    self.next()?;

                    let v = self.parse_expr(true)?;
                    self.expect_tok(Token::OpenBracket)?;

                    let mut branches = vec![];

                    list_helper!(self, ClosedBracket {

                        let pattern = self.parse_pattern()?;
                        self.expect_tok(Token::FatArrow)?;

                        let branch = if self.next_is(Token::OpenBracket)? {
                            self.next()?;
                            let stmts = self.parse_statements()?;
                            self.expect_tok(Token::ClosedBracket)?;
                            MatchBranchCode::Block(stmts)
                        } else {
                            let expr = self.parse_expr(true)?;
                            MatchBranchCode::Expr(expr)
                        };

                        branches.push(MatchBranch { pattern, code: branch });
                    });
                    ExprType::Match { value: v, branches }
                },
                #[cfg(debug_assertions)]
                Token::Dbg => {
                    self.next()?;

                    let show_ptr = self.skip_tok(Token::Mult)?;

                    ExprType::Dbg(self.parse_expr(true)?, show_ptr)
                },

                t @ (Token::Unsafe | Token::Ident | Token::Slf | Token::OpenParen) => {
                    let is_unsafe = if t == Token::Unsafe {
                        self.next()?;
                        true
                    } else {
                        false
                    };

                    let next = if is_unsafe { self.peek()? } else { t };

                    match next {
                        Token::Ident | Token::Slf => {
                            self.next()?;

                            let var_name = self.slice_interned();
                            let var_span = self.span();

                            if is_unsafe {
                                dbg!(&self.peek()?);

                                let ret_type = if self.next_is(Token::Arrow)? {
                                    self.next()?;
                                    let r = Some(self.parse_pattern()?);
                                    self.expect_tok(Token::FatArrow)?;
                                    r
                                } else {
                                    self.next()?;
                                    None
                                };

                                let code = MacroCode::Lambda(self.parse_expr(allow_macros)?);

                                ExprType::Macro {
                                    args: vec![MacroArg::Single {
                                        pattern: PatternNode {
                                            typ: Box::new(PatternType::Path {
                                                var: var_name,
                                                path: vec![],
                                            }),
                                            span: var_span,
                                        },
                                        default: None,
                                    }],
                                    code,
                                    ret_pat: ret_type,
                                    is_unsafe,
                                }
                            } else {
                                ExprType::Var(var_name)
                            }
                        },
                        Token::OpenParen => {
                            todo!()
                            // self.next()?;

                            // let mut check = self.clone();
                            // let mut indent = 1;

                            // let after_close = loop {
                            //     match check.next()? {
                            //         Token::OpenParen => indent += 1,
                            //         Token::Eof => {
                            //             return Err(SyntaxError::UnmatchedToken {
                            //                 not_found: Token::RParen,
                            //                 for_char: Token::OpenParen,
                            //                 area: self.make_area(start),
                            //             })
                            //         },
                            //         Token::RParen => {
                            //             indent -= 1;
                            //             if indent == 0 {
                            //                 break check.next()?;
                            //             }
                            //         },
                            //         _ => (),
                            //     }
                            // };

                            // self.deprecated_features.extended(check.deprecated_features);

                            // match after_close {
                            //     Token::FatArrow | Token::OpenBracket | Token::Arrow
                            //         if allow_macros => {},
                            //     _ => {
                            //         if self.next_is(Token::RParen)? {
                            //             self.next()?;
                            //             break 'out_expr ExprType::Empty;
                            //         }
                            //         let mut inner = self.parse_expr(true)?;
                            //         // println!("hullub");
                            //         self.expect_tok(Token::RParen)?;
                            //         inner.span = start.extended(self.span());
                            //         return Ok(inner);
                            //     },
                            // }

                            // let mut args = vec![];

                            // let mut first_spread_span = None;

                            // let mut index = 0;

                            // list_helper!(
                            //     self,
                            //     is_first,
                            //     RParen {
                            //         let is_spread = self.skip_tok(Token::Spread)?;

                            //         if is_spread {
                            //             if let Some(prev_s) = first_spread_span {
                            //                 return Err(SyntaxError::MultipleSpreadArguments {
                            //                     area: self.make_area(self.span()),
                            //                     prev_area: self.make_area(prev_s)
                            //                 })
                            //             } else {
                            //                 first_spread_span = Some(self.span());
                            //             }
                            //         }

                            //         let pat = self.parse_pattern()?;

                            //         if pat.pat.is_self(&self.interner) {
                            //             if !is_first {
                            //                 return Err(SyntaxError::SelfArgumentNotFirst {
                            //                     area: self.make_area(self.span()),
                            //                     pos: index,
                            //                 })
                            //             }
                            //             if is_spread {
                            //                 return Err(SyntaxError::SelfArgumentCannotBeSpread{
                            //                     area: self.make_area(self.span())
                            //                 })
                            //             }
                            //         }

                            //         if is_spread {
                            //             args.push(MacroArg::Spread { pattern: pat })
                            //         } else {
                            //             let default = if self.skip_tok(Token::Assign)? {
                            //                 Some(self.parse_expr(true)?)
                            //             } else {
                            //                 None
                            //             };
                            //             args.push(MacroArg::Single { pattern: pat, default })
                            //         }
                            //         // println!("fulcrum e");

                            //         index += 1;

                            //     }
                            // );

                            // let ret_type = if self.next_is(Token::Arrow)? {
                            //     self.next()?;
                            //     Some(self.parse_pattern()?)
                            // } else {
                            //     None
                            // };

                            // let code = if self.next_is(Token::FatArrow)? {
                            //     self.next()?;
                            //     MacroCode::Lambda(self.parse_expr(allow_macros)?)
                            // } else {
                            //     MacroCode::Normal(self.parse_block()?)
                            // };

                            // ExprType::Macro {
                            //     args,
                            //     code,
                            //     ret_pat: ret_type,
                            //     is_unsafe,
                            // }
                        },
                        _ => unreachable!(),
                    }
                },
                unary_op
                    if {
                        unary = unary_prec(unary_op);
                        unary.is_some()
                    } =>
                {
                    self.next()?;
                    let unary_prec = unary.unwrap();
                    let next_prec = operators::next_infix(unary_prec);
                    let val = match next_prec {
                        Some(next_prec) => self.parse_op(next_prec, allow_macros)?,
                        None => self.parse_value(allow_macros)?,
                    };

                    ExprType::Unary(unary_op.to_unary_op().unwrap(), val)
                },

                other => {
                    return Err(SyntaxError::UnexpectedToken {
                        expected: "expression".into(),
                        found: other,
                        area: self.make_area(start),
                    })
                },
            }
        };
        let span = start.extended(self.span());

        // self.check_attributes(&attrs, Some(expr.value.target().spanned(expr.span)))?;

        Ok(ExprNode {
            typ: Box::new(expr),
            span,
        })
    }

    pub fn parse_value(&mut self, allow_macros: bool) -> ParseResult<ExprNode> {
        let mut value = self.parse_unit(allow_macros)?;

        loop {
            let prev_span = value.span;

            let typ = match self.peek_strict()? {
                Token::OpenSqBracket => {
                    self.next()?;
                    let index = self.parse_expr(true)?;
                    self.expect_tok(Token::ClosedSqBracket)?;

                    ExprType::Index { base: value, index }
                },
                Token::QMark => {
                    self.next()?;

                    ExprType::Maybe(Some(value))
                },
                Token::ExclMark => {
                    self.next()?;

                    ExprType::TriggerFuncCall(value)
                },
                Token::If => {
                    // if there is a newline, treat as separate statement
                    if self.peek_strict()? == Token::Newline {
                        break;
                    }
                    self.next()?;
                    let cond = self.parse_expr(allow_macros)?;
                    self.expect_tok(Token::Else)?;
                    let if_false = self.parse_expr(allow_macros)?;

                    ExprType::Ternary {
                        cond,
                        if_true: value,
                        if_false,
                    }
                },
                Token::Is => {
                    self.next()?;
                    let pat = self.parse_pattern()?;

                    ExprType::Is(value, pat)
                },
                Token::OpenParen => {
                    todo!()
                    // self.next()?;

                    // let mut params = vec![];
                    // let mut named_params: Vec<(Spanned<Spur>, ExprNode)> = vec![];

                    // let mut parsing_named = None;

                    // list_helper!(self, RParen {
                    //     if self.next_are(&[Token::Ident, Token::Assign])? {
                    //         self.next()?;
                    //         let start = self.span();
                    //         let name = self.slice_interned();

                    //         if let Some((prev, _)) = named_params.iter().find(|(s, _)| s.value == name) {
                    //             return Err(SyntaxError::DuplicateKeywordArg { name: self.resolve(&name).to_string(), prev_area: self.make_area(prev.span), area: self.make_area(self.span()) })
                    //         }

                    //         self.next()?;

                    //         let value = self.parse_expr(true)?;
                    //         parsing_named = Some(start.extended(self.span()));

                    //         named_params.push((name, value));
                    //     } else {

                    //         let value = self.parse_expr(true)?;

                    //         if let Some(s) = parsing_named {
                    //             return Err(SyntaxError::PositionalArgAfterKeyword { keyword_area: self.make_area(s), area: self.make_area(value.span) })
                    //         }

                    //         params.push(value);
                    //     }
                    // });

                    // ExprType::Call {
                    //     base: value,
                    //     params,
                    //     named_params,
                    // }
                },
                _ => match self.peek()? {
                    Token::Dot => {
                        self.next()?;
                        match self.next()? {
                            Token::Ident => {
                                let name = self.slice_interned();
                                ExprType::Member {
                                    base: value,
                                    name,
                                    span: self.span(),
                                }
                            },
                            Token::TypeIndicator => {
                                let name = self.slice()[1..].to_string();
                                ExprType::TypeMember {
                                    base: value,
                                    name: self.intern(name),
                                    span: self.span(),
                                }
                            },
                            Token::Type => ExprType::Typeof(value),
                            other => {
                                return Err(SyntaxError::UnexpectedToken {
                                    expected: "member name".into(),
                                    found: other,
                                    area: self.make_area(self.span()),
                                })
                            },
                        }
                    },
                    Token::DoubleColon => {
                        self.next()?;
                        match self.next()? {
                            Token::Ident => {
                                let name = self.slice_interned();
                                ExprType::Associated {
                                    base: value,
                                    name,
                                    span: self.span(),
                                }
                            },
                            Token::OpenBracket => {
                                todo!()
                                // let items = self.parse_dictlike(true)?;
                                // ExprType::Instance { base: value, items }
                            },
                            other => {
                                return Err(SyntaxError::UnexpectedToken {
                                    expected: "associated member name or instance fields".into(),
                                    found: other,
                                    area: self.make_area(self.span()),
                                })
                            },
                        }
                    },
                    // Token::C
                    _ => break,
                },
            };
            value = ExprNode {
                typ: Box::new(typ),
                span: prev_span.extended(self.span()),
            }
        }

        Ok(value)
    }

    pub fn parse_expr(&mut self, allow_macros: bool) -> ParseResult<ExprNode> {
        self.parse_op(0, allow_macros)
    }

    pub fn parse_op(&mut self, prec: usize, allow_macros: bool) -> ParseResult<ExprNode> {
        let next_prec = operators::next_infix(prec);

        let mut left = match next_prec {
            Some(next_prec) => self.parse_op(next_prec, allow_macros)?,
            None => self.parse_value(allow_macros)?,
        };

        while operators::is_infix_prec(self.peek()?, prec) {
            let op = self.next()?;

            let right = if operators::prec_type(prec) == operators::OpType::Left {
                match next_prec {
                    Some(next_prec) => self.parse_op(next_prec, allow_macros)?,
                    None => self.parse_value(allow_macros)?,
                }
            } else {
                self.parse_op(prec, allow_macros)?
            };
            let new_span = left.span.extended(right.span);
            left = ExprNode {
                typ: Box::new(ExprType::Op(left, op.to_bin_op().unwrap(), right)),
                span: new_span,
            }
        }

        Ok(left)
    }
}
