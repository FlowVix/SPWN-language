use std::rc::Rc;

use self::opcodes::Opcode;
use crate::source::{CodeSpan, SpwnSource};
use crate::util::ImmutVec;
use crate::vm::value::Value;

// pub mod constant;
pub mod opcodes;

pub struct Function {
    pub opcodes: ImmutVec<Opcode>,
    pub opcode_spans: ImmutVec<CodeSpan>,
}

pub struct Bytecode {
    pub consts: ImmutVec<Value>,
    pub funcs: ImmutVec<Function>,
    pub src: Rc<SpwnSource>,
}
