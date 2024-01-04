use self::stmt::StmtNode;

pub mod expr;
pub mod pattern;
pub mod stmt;

#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<StmtNode>,
    // pub file_attributes: Vec<Attribute>,
}
