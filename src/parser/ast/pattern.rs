use ahash::AHashMap;
use lasso::Spur;

use super::expr::ExprNode;
use crate::source::CodeSpan;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum PatternType {
    /// _
    Any,

    /// @<type>
    Type(Spur),
    /// <pattern> | <pattern>
    Either(PatternNode, PatternNode),
    /// <pattern> & <pattern>, <pattern>: <pattern>
    Both(PatternNode, PatternNode),

    /// == <expr>
    Eq(ExprNode),
    /// != <expr>
    Neq(ExprNode),
    /// < <expr>
    Lt(ExprNode),
    /// <= <expr>
    Lte(ExprNode),
    /// > <expr>
    Gt(ExprNode),
    /// >= <expr>
    Gte(ExprNode),

    In(ExprNode),

    /// <pattern>[<pattern>]
    ArrayPattern(PatternNode, PatternNode),
    /// <pattern>{:}
    DictPattern(PatternNode),

    /// [ <pattern> ]
    ArrayDestructure(Vec<PatternNode>),

    /// { key: <pattern> ... }
    DictDestructure(AHashMap<Spur, (PatternNode, CodeSpan)>),
    /// <pattern>? or ?
    MaybeDestructure(Option<PatternNode>),
    /// @typ::{ <key>(: <pattern>)? ... }
    InstanceDestructure(Spur, AHashMap<Spur, (PatternNode, CodeSpan)>),

    /// ()
    Empty,

    /// index, member, associated member
    Path {
        var: Spur,
        path: Vec<AssignPath>,
    },
    /// mut var
    Mut {
        name: Spur,
    },
    /// &var
    Ref {
        name: Spur,
    },

    /// <pattern> if <expr>
    IfGuard {
        pat: PatternNode,
        cond: ExprNode,
    },

    /// (<pattern>...) -> <pattern>
    MacroPattern {
        args: Vec<PatternNode>,
        ret: PatternNode,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct PatternNode {
    pub typ: Box<PatternType>,
    pub span: CodeSpan,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum AssignPath {
    Index(ExprNode),
    Member(Spur),
    Associated(Spur),
}
