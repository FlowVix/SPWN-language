use lasso::Spur;

use super::expr::ExprNode;
use super::pattern::PatternNode;
use crate::parser::operators::AssignOp;
use crate::source::CodeSpan;

#[derive(Debug)]
pub struct StmtNode {
    pub typ: Box<StmtType>,
    pub span: CodeSpan,
}

#[derive(Debug)]
pub enum StmtType {
    Expr(ExprNode),
    Assign(PatternNode, ExprNode),
    AssignOp(PatternNode, AssignOp, ExprNode),
    If {
        branches: Vec<(ExprNode, Statements)>,
        else_branch: Option<Statements>,
    },
    While {
        cond: ExprNode,
        code: Statements,
    },
    For {
        iter: PatternNode,
        iterator: ExprNode,
        code: Statements,
    },
    TryCatch {
        try_code: Statements,
        catch_pat: Option<PatternNode>,
        catch_code: Statements,
    },
    Arrow(StmtNode),

    Return(Option<ExprNode>),
    Break,
    Continue,

    Unsafe(Statements),

    TypeDef {
        name: Spur,
        public: bool,
    },

    // Impl {
    //     name: Spur,
    //     name_span: CodeSpan,
    //     items: Vec<MapDictItem>,
    // },
    // Overload {
    //     op: Operator,
    //     macros: Vec<Vis<ExprNode>>,
    // },
    Throw(ExprNode),
}

pub type Statements = Vec<StmtNode>;
