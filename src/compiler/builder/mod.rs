pub mod proto;

use self::proto::{Block, BlockContent, BlockID, ProtoBytecode, ProtoOpcode};
use super::CompileResult;
use crate::bytecode::constant::Constant;
use crate::bytecode::opcode::{Opcode, VarID};
use crate::bytecode::CallExpr;
// use crate::bytecode::pattern::RuntimePattern;
use crate::source::CodeSpan;

pub struct CodeBuilder<'a> {
    pub proto_bytecode: &'a mut ProtoBytecode,
    pub func: usize,
    pub block: BlockID,
}

impl<'a> CodeBuilder<'a> {
    fn current_block(&mut self) -> &mut Block {
        &mut self.proto_bytecode.blocks[self.block]
    }

    // pub fn enter_unsafe(&mut self) {
    //     self.proto_bytecode.functions[self.func].unsafe_level += 1
    // }

    // pub fn exit_unsafe(&mut self) {
    //     self.proto_bytecode.functions[self.func].unsafe_level -= 1
    // }

    pub fn pop_top(&mut self, span: CodeSpan) {
        self.push_raw_opcode(Opcode::PopTop, span);
    }

    pub fn set_var(&mut self, v: VarID, span: CodeSpan) {
        self.push_raw_opcode(Opcode::SetVar(v), span);
    }

    pub fn load_var(&mut self, v: VarID, span: CodeSpan) {
        self.push_raw_opcode(Opcode::LoadVar(v), span);
    }

    pub fn new_block<F: FnOnce(&mut CodeBuilder) -> CompileResult<()>>(
        &mut self,
        f: F,
    ) -> CompileResult<()> {
        let f_block = self.proto_bytecode.blocks.insert(Default::default());

        self.current_block()
            .content
            .push(BlockContent::Block(f_block));

        f(&mut CodeBuilder {
            block: f_block,
            func: self.func,
            proto_bytecode: self.proto_bytecode,
        })
    }

    pub fn next_var(&mut self) -> VarID {
        let r = self.proto_bytecode.functions[self.func].var_count.into();
        self.proto_bytecode.functions[self.func].var_count += 1;
        r
    }

    pub fn push_opcode(&mut self, opcode: ProtoOpcode, span: CodeSpan) {
        self.current_block()
            .content
            .push(BlockContent::Opcode(opcode, span))
    }

    pub fn push_raw_opcode(&mut self, opcode: Opcode, span: CodeSpan) {
        self.push_opcode(ProtoOpcode::Raw(opcode), span)
    }

    pub fn load_const(&mut self, c: Constant, span: CodeSpan) {
        let id = self.proto_bytecode.consts.insert(c).into();
        self.push_opcode(ProtoOpcode::Raw(Opcode::LoadConst(id)), span)
    }

    pub fn ret(&mut self, no_value: bool, span: CodeSpan) {
        if no_value {
            self.load_const(Constant::Empty, span);
        }
        self.push_raw_opcode(Opcode::Return, span);
    }

    // pub fn check_pattern(&mut self, p: RuntimePattern, span: CodeSpan) {
    //     let id = self.proto_bytecode.patterns.insert(p).into();
    //     self.push_opcode(ProtoOpcode::Raw(Opcode::CheckPattern(id)), span)
    // }
    pub fn do_call(&mut self, c: CallExpr, span: CodeSpan) {
        let id = self.proto_bytecode.call_exprs.insert(c).into();
        self.push_opcode(ProtoOpcode::Raw(Opcode::Call(id)), span)
    }
}
