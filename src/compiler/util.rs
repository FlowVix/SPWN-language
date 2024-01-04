use super::error::CompilerError;
use super::{CompileResult, Compiler, ScopeID};
use crate::parser::ast::expr::{ExprNode, ExprType};

impl<'a> Compiler<'a> {
    pub fn is_mut_expr(&self, expr: &ExprNode, scope: ScopeID) -> CompileResult<bool> {
        Ok(match &*expr.typ {
            ExprType::Var(v) => self.get_var_or_err(*v, scope, expr.span)?.mutable,
            // ExprType::Index { base, .. } => self.is_mut_expr(base, scope)?,
            // ExprType::Member { base, .. } => self.is_mut_expr(base, scope)?,
            // ExprType::Associated { base, .. } => self.is_mut_expr(base, scope)?,
            // ExprType::Ternary {
            //     if_true, if_false, ..
            // } => self.is_mut_expr(if_true, scope)? && self.is_mut_expr(if_false, scope)?,
            _ => false,
        })
    }
}
