use ahash::AHashMap;
use lasso::Spur;

use super::expr::ExprNode;
use crate::source::CodeSpan;

#[derive(Debug)]
pub struct PatternNode {
    pub typ: Box<PatternType>,
    pub span: CodeSpan,
}

#[derive(Debug)]
pub enum PatternType {
    /// _
    Any,

    /// @<type>
    Type(Spur),
    /// \<pattern> | \<pattern>
    Either(PatternNode, PatternNode),
    /// \<pattern> & \<pattern>, \<pattern>: \<pattern>
    Both(PatternNode, PatternNode),

    /// == \<expr>
    Eq(ExprNode),
    /// != <expr>
    NEq(ExprNode),
    /// \< \<expr>
    Lt(ExprNode),
    /// \<= \<expr>
    LtE(ExprNode),
    /// \> \<expr>
    Gt(ExprNode),
    /// \>= \<expr>
    GtE(ExprNode),

    /// in \<expr>
    In(ExprNode),

    /// \<pattern>[\<pattern>]
    ArrayPattern(PatternNode, Option<PatternNode>),
    /// \<pattern>{}
    DictPattern(PatternNode),

    /// \[ \<pattern> ... ]
    ArrayDestructure(Vec<PatternNode>),

    /// { key(: \<pattern>)? ... }
    DictDestructure(AHashMap<Spur, (Option<PatternNode>, CodeSpan)>),
    /// \<pattern>? or ?
    MaybeDestructure(Option<PatternNode>),
    /// @typ::{ \<key>(: \<pattern>)? ... }
    InstanceDestructure(Spur, AHashMap<Spur, (Option<PatternNode>, CodeSpan)>),

    /// ()
    Empty,

    /// index, member, associated member
    Path { var: Spur, path: Vec<AssignPath> },
    /// mut var
    Mut { name: Spur },
    /// &var
    Ref { name: Spur },

    /// \<pattern> if \<expr>
    IfGuard { pat: PatternNode, cond: ExprNode },
    // /// (<pattern>...) -> <pattern>
    // MacroPattern {
    //     args: Vec<PatternNode>,
    //     ret: PatternNode,
    // },
    /// A malformed pattern or a pattern in an invalid location
    Err,
}

#[derive(Debug)]
pub enum AssignPath {
    Index(ExprNode),
    Member(Spur),
    Associated(Spur),
}

impl PatternType {
    pub fn get_name(&self) -> Option<Spur> {
        match self {
            PatternType::Mut { name } => Some(*name),
            PatternType::Ref { name } => Some(*name),
            PatternType::Path { var, path, .. } if path.is_empty() => Some(*var),
            PatternType::Both(a, ..) => a.typ.get_name(),
            _ => None,
        }
    }

    pub fn needs_mut(&self) -> bool {
        match self {
            PatternType::Either(a, b) | PatternType::Both(a, b) => {
                a.typ.needs_mut() || b.typ.needs_mut()
            },
            PatternType::ArrayPattern(elem, ..) | PatternType::DictPattern(elem) => {
                elem.typ.needs_mut()
            },
            PatternType::ArrayDestructure(v) => v.iter().any(|p| p.typ.needs_mut()),
            PatternType::DictDestructure(map) => map
                .iter()
                .any(|(_, (p, _))| p.as_ref().is_some_and(|p| p.typ.needs_mut())),
            PatternType::InstanceDestructure(_, map) => map
                .iter()
                .any(|(_, (p, _))| p.as_ref().is_some_and(|p| p.typ.needs_mut())),
            PatternType::MaybeDestructure(v) => v.as_ref().is_some_and(|p| p.typ.needs_mut()),

            PatternType::IfGuard { pat, .. } => pat.typ.needs_mut(),
            PatternType::Ref { .. } => true,
            _ => false,
        }
    }
}
