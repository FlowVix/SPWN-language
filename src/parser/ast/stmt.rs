use allow_until::AllowUntil;
use delve::EnumToStr;
use lasso::Spur;

use super::expr::ExprNode;
use super::MapDictItem;
use crate::source::CodeSpan;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone, EnumToStr, AllowUntil)]
pub enum StmtType {
    Expr(ExprNode),
    // Assign(PatternNode, ExprNode),
    // AssignOp(PatternNode, AssignOp, ExprNode),
    If {
        branches: Vec<(ExprNode, Statements)>,
        else_branch: Option<Statements>,
    },
    While {
        cond: ExprNode,
        code: Statements,
    },
    // For {
    //     iter: PatternNode,
    //     iterator: ExprNode,
    //     code: Statements,
    // },
    // TryCatch {
    //     try_code: Statements,
    //     catch_pat: Option<PatternNode>,
    //     catch_code: Statements,
    // },
    Arrow(Box<StmtNode>),

    Return(Option<ExprNode>),
    Break,
    Continue,

    Unsafe(Statements),

    TypeDef {
        name: Spur,
        public: bool,
    },

    Impl {
        name: Spur,
        name_span: CodeSpan,
        items: Vec<MapDictItem>,
    },
    // Overload {
    //     op: Operator,
    //     macros: Vec<Vis<ExprNode>>,
    // },
    Throw(ExprNode),
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct StmtNode {
    pub typ: Box<StmtType>,
    // pub attributes: Vec<Attribute>,
    pub span: CodeSpan,
}

pub type Statements = Vec<StmtNode>;
