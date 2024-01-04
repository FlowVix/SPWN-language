use self::constant::Constant;
use self::opcode::Opcode;
// use self::pattern::RuntimePattern;
use crate::source::{CodeSpan, SpwnSource};
use crate::util::{ImmutStr, ImmutVec};

pub mod constant;
pub mod debug;
pub mod opcode;

#[derive(Debug)]
pub struct Bytecode {
    pub funcs: ImmutVec<Function>,
    pub consts: ImmutVec<Constant>,
    pub call_exprs: ImmutVec<CallExpr>,

    // pub patterns: ImmutVec<RuntimePattern>,
    pub src: &'static SpwnSource,
}

#[derive(Debug)]
pub struct Function {
    pub opcodes: ImmutVec<(Opcode, CodeSpan)>,
    pub var_count: u16,

    pub args: ImmutVec<FuncArg>,
}

#[derive(Debug, Clone)]
pub struct FuncArg {
    pub name: Option<ImmutStr>,
    pub needs_mut: bool,
    pub span: CodeSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub positional: ImmutVec<bool>,
    // pub named: ImmutVec<(ImmutStr, bool)>,
}
