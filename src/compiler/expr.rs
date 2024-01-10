use itertools::Itertools;

use super::builder::CodeBuilder;
use super::error::CompilerError;
use super::{CompileResult, Compiler, Scope, ScopeID, ScopeType};
use crate::bytecode::constant::Constant;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::{CallExpr, FuncArg};
use crate::parser::ast::expr::{ExprNode, ExprType, MacroBody};
use crate::parser::operators::{BinOp, UnaryOp};
use crate::source::CodeSpan;

impl<'a> Compiler<'a> {
    pub fn compile_expr(
        &mut self,
        expr: &'a ExprNode,
        scope: ScopeID,
        builder: &mut CodeBuilder,
    ) -> CompileResult<()> {
        match &*expr.typ {
            ExprType::Int(v) => {
                builder.load_const(Constant::Int(*v), expr.span);
            },
            ExprType::Float(v) => {
                builder.load_const(Constant::Float(*v), expr.span);
            },
            ExprType::Bool(v) => {
                builder.load_const(Constant::Bool(*v), expr.span);
            },
            ExprType::Array(v) => {
                for i in v {
                    self.compile_expr(i, scope, builder)?;
                }

                builder.push_raw_opcode(
                    Opcode::MakeArray {
                        len: v.len() as u16,
                    },
                    expr.span,
                );
            },
            ExprType::BinOp(a, op, b) => {
                self.compile_expr(a, scope, builder)?;
                self.compile_expr(b, scope, builder)?;

                macro_rules! push {
                    ($name:ident) => {
                        builder.push_raw_opcode(Opcode::$name, expr.span);
                    };
                }

                match op {
                    BinOp::Eq => push!(Eq),
                    BinOp::NEq => push!(NEq),
                    BinOp::Gt => push!(Gt),
                    BinOp::GtE => push!(GtE),
                    BinOp::Lt => push!(Lt),
                    BinOp::LtE => push!(LtE),
                    BinOp::Plus => push!(Plus),
                    BinOp::Minus => push!(Minus),
                    BinOp::Mult => push!(Mult),
                    BinOp::Div => push!(Div),
                    BinOp::Mod => push!(Mod),
                    BinOp::Pow => push!(Pow),
                }
            },
            ExprType::UnaryOp(op, v) => {
                self.compile_expr(v, scope, builder)?;

                macro_rules! push {
                    ($name:ident) => {
                        builder.push_raw_opcode(Opcode::$name, expr.span)
                    };
                }

                match op {
                    UnaryOp::ExclMark => push!(UnaryNot),
                    UnaryOp::Minus => push!(UnaryMinus),
                }
            },
            ExprType::Var(v) => {
                builder.load_var(self.get_var_or_err(*v, scope, expr.span)?.id, expr.span);
            },
            ExprType::Dbg(e) => {
                self.compile_expr(e, scope, builder)?;
                builder.push_raw_opcode(Opcode::Dbg, expr.span);
            },
            ExprType::Macro {
                body,
                args: arg_pats,
                ret_pat,
                body_span,
            } => {
                let args = arg_pats
                    .iter()
                    .map(|p| FuncArg {
                        name: p.typ.get_name().map(|v| self.resolve_immut(&v)),
                        needs_mut: p.typ.needs_mut(),
                        span: p.span,
                    })
                    .collect_vec();
                let f_id = builder.proto_bytecode.new_func(
                    |builder| {
                        let base_scope = self.scopes.insert(Scope {
                            vars: Default::default(),
                            parent: None,
                            typ: Some(ScopeType::MacroBody(ret_pat.as_ref())),
                        });

                        for p in arg_pats {
                            self.compile_pattern_check(p, true, base_scope, builder)?;
                            builder.push_raw_opcode(Opcode::MismatchThrowIfFalse, p.span);
                        }

                        match body {
                            MacroBody::Normal(s) => {
                                self.compile_stmts(s, base_scope, builder)?;
                            },
                            MacroBody::Lambda(e) => {
                                self.compile_expr(e, base_scope, builder)?;
                                builder.ret(false, *body_span);
                            },
                        };
                        Ok(())
                    },
                    args,
                )?;
                builder.push_raw_opcode(Opcode::MakeMacro(f_id), expr.span);
            },
            ExprType::Call { base, params } => {
                self.compile_expr(base, scope, builder)?;
                for i in params {
                    self.compile_expr(i, scope, builder)?;
                }
                builder.do_call(
                    CallExpr {
                        positional: params
                            .iter()
                            .map(|e| self.is_mut_expr(e, scope))
                            .collect::<Result<Vec<_>, _>>()?
                            .into(),
                    },
                    expr.span,
                );
            },
            ExprType::Err => unreachable!(),
        };
        Ok(())
    }
}
