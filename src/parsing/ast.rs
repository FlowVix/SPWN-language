use lasso::Spur;

use crate::{lexing::tokens::Token, sources::CodeSpan};

use super::{
    attributes::{ExprAttribute, ScriptAttribute, StmtAttribute},
    utils::operators::{BinOp, UnaryOp},
};

#[derive(Debug, Clone)]
pub enum ImportType {
    Module(Spur),
    Library(Spur),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum IDClass {
    Group = 0,
    Color = 1,
    Block = 2,
    Item = 3,
}

#[derive(Debug, Clone)]
pub enum MacroCode {
    Normal(Statements),
    Lambda(ExprNode),
}

#[derive(Debug, Clone)]
pub struct ExprNode {
    pub expr: Box<Expression>,
    pub attributes: Vec<ExprAttribute>,
    pub span: CodeSpan,
}

#[derive(Debug, Clone)]
pub struct StmtNode {
    pub stmt: Box<Statement>,
    pub attributes: Vec<StmtAttribute>,
    pub span: CodeSpan,
}

pub type DictItems = Vec<(Spanned<String>, Option<ExprNode>)>;

#[derive(Debug, Clone, strum::IntoStaticStr)]
pub enum Expression {
    Int(i64),
    Float(f64),
    String(Spur),
    Bool(bool),

    Id(IDClass, Option<u16>),

    Op(ExprNode, BinOp, ExprNode),
    Unary(UnaryOp, ExprNode),

    Var(Spur),
    Type(Spur),

    Array(Vec<ExprNode>),
    Dict(DictItems),

    Maybe(Option<ExprNode>),

    Index {
        base: ExprNode,
        index: ExprNode,
    },
    Member {
        base: ExprNode,
        name: Spur,
    },
    Associated {
        base: ExprNode,
        name: Spur,
    },

    Call {
        base: ExprNode,
        params: Vec<ExprNode>,
        named_params: Vec<(Spur, ExprNode)>,
    },

    Macro {
        args: Vec<(Spanned<Spur>, Option<ExprNode>, Option<ExprNode>)>,
        ret_type: Option<ExprNode>,
        code: MacroCode,
    },
    MacroPattern {
        args: Vec<ExprNode>,
        ret_type: ExprNode,
    },

    TriggerFunc {
        attributes: Vec<ExprAttribute>,
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

    Import(ImportType),

    Instance {
        base: ExprNode,
        items: DictItems,
    },
}

#[derive(Debug, Clone, strum::IntoStaticStr)]
pub enum Statement {
    Expr(ExprNode),
    Let(ExprNode, ExprNode),
    If {
        branches: Vec<(ExprNode, Statements)>,
        else_branch: Option<Statements>,
    },
    While {
        cond: ExprNode,
        code: Statements,
    },
    For {
        iter: ExprNode,
        iterator: ExprNode,
        code: Statements,
    },
    TryCatch {
        try_code: Statements,
        error_var: Option<Spur>,
        catch_code: Statements,
    },

    Arrow(Box<Statement>),

    Return(Option<ExprNode>),
    Break,
    Continue,

    TypeDef(Spur),
    Extract(ExprNode),

    Impl {
        typ: Spur,
        items: DictItems,
    },
}

pub type Statements = Vec<StmtNode>;

impl Expression {
    pub fn into_node(self, attributes: Vec<ExprAttribute>, span: CodeSpan) -> ExprNode {
        ExprNode {
            expr: Box::new(self),
            attributes,
            span,
        }
    }
}
impl Statement {
    pub fn into_node(self, attributes: Vec<StmtAttribute>, span: CodeSpan) -> StmtNode {
        StmtNode {
            stmt: Box::new(self),
            attributes,
            span,
        }
    }
}

impl ExprNode {
    pub fn extended(self, other: CodeSpan) -> Self {
        Self {
            span: self.span.extend(other),
            ..self
        }
    }
}
impl StmtNode {
    pub fn extended(self, other: CodeSpan) -> Self {
        Self {
            span: self.span.extend(other),
            ..self
        }
    }
}
#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<StmtNode>,
    pub file_attributes: Vec<ScriptAttribute>,
}

#[derive(Clone, Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: CodeSpan,
}
impl<T> Spanned<T> {
    pub fn split(self) -> (T, CodeSpan) {
        (self.value, self.span)
    }
    pub fn extended(self, other: CodeSpan) -> Self {
        Self {
            span: self.span.extend(other),
            ..self
        }
    }
    pub fn apply_fn<U, F: FnOnce(T) -> U>(self, f: F) -> Spanned<U> {
        f(self.value).spanned(self.span)
    }
}

pub trait Spannable {
    fn spanned(self, span: CodeSpan) -> Spanned<Self>
    where
        Self: Sized;
}

impl<T> Spannable for T {
    fn spanned(self, span: CodeSpan) -> Spanned<Self>
    where
        Self: Sized,
    {
        Spanned { value: self, span }
    }
}
