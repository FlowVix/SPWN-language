use delve::EnumToStr;
use lasso::Spur;

use super::pattern::PatternNode;
use super::stmt::Statements;
use super::MapDictItem;
use crate::gd::ids::IDClass;
use crate::parser::operators::{BinOp, UnaryOp};
use crate::source::CodeSpan;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum MatchBranchCode {
    Expr(ExprNode),
    Block(Statements),
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct MatchBranch {
    pub pattern: PatternNode,
    pub code: MatchBranchCode,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum MacroCode {
    Normal(Statements),
    Lambda(ExprNode),
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum MacroArg {
    Single {
        pattern: PatternNode,
        default: Option<ExprNode>,
    },
    Spread {
        pattern: PatternNode,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone, EnumToStr)]
pub enum ExprType {
    Int(i64),
    Float(f64),
    String(Spur),
    Bool(bool),

    Id {
        class: IDClass,
        value: Option<u16>,
    },
    Op(ExprNode, BinOp, ExprNode),
    Unary(UnaryOp, ExprNode),

    Var(Spur),
    Type(Spur),

    Array(Vec<ExprNode>),
    Dict(Vec<MapDictItem>),

    Maybe(Option<ExprNode>),

    Is(ExprNode, PatternNode),
    Index {
        base: ExprNode,
        index: ExprNode,
    },
    Member {
        base: ExprNode,
        name: Spur,
        span: CodeSpan,
    },
    TypeMember {
        base: ExprNode,
        name: Spur,
        span: CodeSpan,
    },
    Associated {
        base: ExprNode,
        name: Spur,
        span: CodeSpan,
    },

    // Call {
    //     base: ExprNode,
    //     params: Vec<ExprNode>,
    //     named_params: Vec<(Spanned<Spur>, ExprNode)>,
    // },
    Macro {
        args: Vec<MacroArg>,
        ret_pat: Option<PatternNode>,
        code: MacroCode,
        is_unsafe: bool,
    },
    TriggerFunc {
        code: Statements,
    },

    TriggerFuncCall(ExprNode),

    Ternary {
        cond: ExprNode,
        if_true: ExprNode,
        if_false: ExprNode,
    },

    Typeof(ExprNode),

    Builtins,
    Empty,
    Epsilon,

    // Import(Import),
    // ExtractImport {
    //     import: Import,
    //     destructure: Option<Spanned<AHashMap<Spanned<ModuleDestructureKey>, Option<PatternNode>>>>,
    // },
    #[cfg(debug_assertions)]
    Dbg(ExprNode, bool),

    Instance {
        base: ExprNode,
        items: Vec<MapDictItem>,
    },
    // Obj(ObjectType, Vec<(Spanned<ObjKeyType>, ExprNode)>),
    Match {
        value: ExprNode,
        branches: Vec<MatchBranch>,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct ExprNode {
    pub typ: Box<ExprType>,
    // pub attributes: Vec<Attribute>,
    pub span: CodeSpan,
}
