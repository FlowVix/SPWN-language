use lasso::Spur;

use self::expr::ExprNode;
use crate::source::CodeSpan;

pub mod expr;
pub mod pattern;
pub mod stmt;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum MapDictItem {
    NameKey {
        key: Spur,
        key_span: CodeSpan,
        value: Option<ExprNode>,
    },
    ValueKey {
        key: ExprNode,
        value: ExprNode,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct TypeDictItem {
    pub name: Spur,
    pub name_span: CodeSpan,
    pub value: Option<ExprNode>,
    pub public: bool,
}
