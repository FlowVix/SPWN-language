use lasso::Spur;

use super::pattern::PatternNode;
use super::stmt::Statements;
use crate::parser::operators::{BinOp, UnaryOp};
use crate::source::CodeSpan;

#[derive(Debug)]
pub struct ExprNode {
    pub typ: Box<ExprType>,
    pub span: CodeSpan,
}

#[derive(Debug)]
pub enum ExprType {
    Int(i64),
    Float(f64),
    Bool(bool),

    Array(Vec<ExprNode>),

    BinOp(ExprNode, BinOp, ExprNode),
    UnaryOp(UnaryOp, ExprNode),

    Dbg(ExprNode),

    Var(Spur),

    Macro {
        body: MacroBody,
        body_span: CodeSpan,
        args: Vec<PatternNode>,
        ret_pat: Option<PatternNode>,
    },

    Call {
        base: ExprNode,
        params: Vec<ExprNode>,
        // named_params: Vec<(Spanned<Spur>, ExprNode)>,
    },

    Err,
}

#[derive(Debug)]
pub enum MacroBody {
    Normal(Statements),
    Lambda(ExprNode),
}
