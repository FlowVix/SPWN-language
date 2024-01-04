use super::builder::proto::{JumpTo, ProtoOpcode};
use super::builder::CodeBuilder;
use super::{CompileResult, Compiler, ScopeID, ScopeType, VarData};
use crate::bytecode::opcode::Opcode;
use crate::parser::ast::stmt::{Statements, StmtNode, StmtType};

impl<'a> Compiler<'a> {
    pub fn compile_stmt(
        &mut self,
        stmt: &'a StmtNode,
        scope: ScopeID,
        builder: &mut CodeBuilder,
    ) -> CompileResult<()> {
        match &*stmt.typ {
            StmtType::Expr(e) => {
                self.compile_expr(e, scope, builder)?;
                builder.pop_top(stmt.span);
            },
            StmtType::Assign(p, e) => {
                self.compile_expr(e, scope, builder)?;
                self.compile_pattern_check(p, false, scope, builder)?;
                builder.push_raw_opcode(Opcode::MismatchThrowIfFalse, stmt.span);
            },
            StmtType::AssignOp(..) => todo!(),
            StmtType::If {
                branches,
                else_branch,
            } => {
                builder.new_block(|builder| {
                    let outer = builder.block;

                    for (cond, code) in branches {
                        builder.new_block(|builder| {
                            let derived = self.derive_scope(scope, None);

                            self.compile_expr(cond, derived, builder)?;
                            builder.push_opcode(
                                ProtoOpcode::JumpIfFalse(JumpTo::End(builder.block)),
                                cond.span,
                            );

                            self.compile_stmts(code, derived, builder)?;

                            builder.push_opcode(ProtoOpcode::Jump(JumpTo::End(outer)), cond.span);
                            Ok(())
                        })?;
                    }

                    if let Some(code) = else_branch {
                        let derived = self.derive_scope(scope, None);

                        self.compile_stmts(code, derived, builder)?;
                    }

                    Ok(())
                })?;
            },
            StmtType::While { cond, code } => {
                let derived = self.derive_scope(scope, Some(ScopeType::Loop(builder.block)));

                builder.new_block(|builder| {
                    self.compile_expr(cond, derived, builder)?;
                    builder.push_opcode(
                        ProtoOpcode::JumpIfFalse(JumpTo::End(builder.block)),
                        cond.span,
                    );

                    self.compile_stmts(code, derived, builder)?;
                    builder.push_opcode(ProtoOpcode::Jump(JumpTo::Start(builder.block)), cond.span);

                    Ok(())
                })?;
            },
            StmtType::For {
                iter,
                iterator,
                code,
            } => todo!(),
            StmtType::TryCatch {
                try_code,
                catch_pat,
                catch_code,
            } => todo!(),
            StmtType::Arrow(s) => {
                builder.new_block(|builder| {
                    let inner_scope =
                        self.derive_scope(scope, Some(ScopeType::ArrowStmt(stmt.span))); // variables made in arrow statements shouldnt be allowed anyways
                    builder.push_opcode(
                        ProtoOpcode::EnterArrowStatement(JumpTo::End(builder.block)),
                        stmt.span,
                    );
                    self.compile_stmt(s, inner_scope, builder)?;
                    builder.push_raw_opcode(Opcode::YeetContext, stmt.span);
                    Ok(())
                })?;
            },
            StmtType::Return(v) => todo!(),
            StmtType::Break => todo!(),
            StmtType::Continue => todo!(),
            StmtType::Unsafe(_) => todo!(),
            StmtType::TypeDef { name, public } => todo!(),
            StmtType::Throw(_) => todo!(),
        }
        Ok(())
    }

    pub fn compile_stmts(
        &mut self,
        stmts: &'a Statements,
        scope: ScopeID,
        builder: &mut CodeBuilder,
    ) -> CompileResult<()> {
        for s in stmts {
            self.compile_stmt(s, scope, builder)?;
        }
        Ok(())
    }
}
