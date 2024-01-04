use ahash::AHashMap;

use super::CodeBuilder;
use crate::bytecode::constant::Constant;
use crate::bytecode::opcode::{FuncID, Opcode};
use crate::bytecode::{Bytecode, CallExpr, FuncArg, Function};
use crate::compiler::CompileResult;
use crate::ids_helper;
use crate::source::{CodeSpan, SpwnSource};
use crate::util::slabmap::SlabMap;
use crate::util::unique_register::UniqueRegister;

ids_helper! {
    BlockID(u16);
}

#[derive(Debug, Clone, Copy)]
pub enum JumpTo {
    Start(BlockID),
    End(BlockID),
}

#[derive(Debug, Clone, Copy)]
pub enum ProtoOpcode {
    Raw(Opcode),
    Jump(JumpTo),
    JumpIfFalse(JumpTo),
    JumpIfTrue(JumpTo),
    UnwrapOrJump(JumpTo),
    // PushTryCatch(Register, JumpTo),
    EnterArrowStatement(JumpTo),
}

#[derive(Debug, Clone, Copy)]
pub enum BlockContent {
    Opcode(ProtoOpcode, CodeSpan),
    Block(BlockID),
}

#[derive(Default, Debug)]
pub struct Block {
    pub content: Vec<BlockContent>,
}

#[derive(Debug)]
pub struct ProtoFunc {
    pub code: BlockID,
    pub var_count: u16,

    pub args: Vec<FuncArg>,
    // span: CodeSpan,
    // args: ImmutVec<Spanned<(Option<ImmutStr>, Mutability)>>,
    // spread_arg: Option<u8>,
    // captured_regs: Vec<(UnoptRegister, UnoptRegister)>,
    // child_funcs: Vec<FuncID>,
    // unsafe_level: usize,
}

#[derive(Debug)]
pub struct ProtoBytecode {
    pub consts: UniqueRegister<Constant>,
    pub call_exprs: UniqueRegister<CallExpr>,
    pub functions: Vec<ProtoFunc>,

    pub blocks: SlabMap<BlockID, Block>,
    // import_paths: UniqueRegister<SpwnSource>,

    // call_exprs: Vec<CallExpr<UnoptRegister, UnoptRegister, ImmutStr>>,
    // debug_funcs: Vec<FuncID>,

    // pub overloads: AHashMap<Operator, Vec<FuncID>>,
}
impl ProtoBytecode {
    pub fn new() -> Self {
        Self {
            consts: UniqueRegister::new(),
            call_exprs: UniqueRegister::new(),
            // patterns: UniqueRegister::new(),
            functions: vec![],
            blocks: SlabMap::new(),
        }
    }

    pub fn new_func<F: FnOnce(&mut CodeBuilder) -> CompileResult<()>>(
        &mut self,
        f: F,
        args: Vec<FuncArg>,
        // span: CodeSpan,
    ) -> CompileResult<FuncID> {
        let f_block = self.blocks.insert(Default::default());
        self.functions.push(ProtoFunc {
            code: f_block,
            var_count: 0,
            args,
        });
        let func = self.functions.len() - 1;
        f(&mut CodeBuilder {
            func,
            proto_bytecode: self,
            block: f_block,
        })?;
        Ok(func.into())
    }

    pub fn build(mut self, src: &'static SpwnSource) -> Bytecode {
        type BlockPos = (u16, u16);

        let consts = self.consts.make_vec();
        let call_exprs = self.call_exprs.make_vec();

        let mut funcs: Vec<Function> = vec![];

        for (func_id, func) in self.functions.iter().enumerate() {
            let mut block_positions = AHashMap::new();
            let mut code_len = 0;

            let mut opcodes: Vec<(Opcode, CodeSpan)> = vec![];

            fn get_block_pos(
                code: &ProtoBytecode,
                block: BlockID,
                length: &mut u16,
                positions: &mut AHashMap<BlockID, BlockPos>,
            ) {
                let start = *length;
                for c in &code.blocks[block].content {
                    match c {
                        BlockContent::Opcode(..) => {
                            *length += 1;
                        },
                        BlockContent::Block(b) => get_block_pos(code, *b, length, positions),
                    }
                }
                let end = *length;
                positions.insert(block, (start, end));
            }

            fn build_block(
                code: &ProtoBytecode,
                block: BlockID,
                opcodes: &mut Vec<(Opcode, CodeSpan)>,
                positions: &AHashMap<BlockID, BlockPos>,
            ) {
                let get_jump_pos = |jump: JumpTo| -> u16 {
                    match jump {
                        JumpTo::Start(path) => positions[&path].0,
                        JumpTo::End(path) => positions[&path].1,
                    }
                };

                for &content in &code.blocks[block].content {
                    match content {
                        BlockContent::Opcode(o, span) => {
                            let opcode = match o {
                                ProtoOpcode::Raw(o) => o,
                                ProtoOpcode::Jump(to) => Opcode::Jump(get_jump_pos(to).into()),
                                ProtoOpcode::JumpIfFalse(to) => {
                                    Opcode::JumpIfFalse(get_jump_pos(to).into())
                                },
                                ProtoOpcode::JumpIfTrue(to) => {
                                    Opcode::JumpIfTrue(get_jump_pos(to).into())
                                },
                                ProtoOpcode::EnterArrowStatement(to) => {
                                    Opcode::EnterArrowStatement(get_jump_pos(to).into())
                                },
                                _ => todo!(),
                                // ProtoOpcode::UnwrapOrJump(r, to) => Opcode::UnwrapOrJump {
                                //     check: r,
                                //     to: get_jump_pos(to).into(),
                                // },
                                // ProtoOpcode::EnterArrowStatement(skip) => {
                                //     Opcode::EnterArrowStatement {
                                //         skip: get_jump_pos(skip).into(),
                                //     }
                                // },
                                // ProtoOpcode::PushTryCatch(reg, to) => Opcode::PushTryCatch {
                                //     reg,
                                //     to: get_jump_pos(to).into(),
                                // },
                            };
                            opcodes.push((opcode, span));
                        },
                        BlockContent::Block(b) => build_block(code, b, opcodes, positions),
                    }
                }
            }

            get_block_pos(&self, func.code, &mut code_len, &mut block_positions);
            build_block(&self, func.code, &mut opcodes, &block_positions);

            funcs.push(Function {
                var_count: func.var_count,
                opcodes: opcodes.into(),
                args: func.args.clone().into(),
            })
        }

        Bytecode {
            funcs: funcs.into(),
            call_exprs: call_exprs.into(),
            consts: consts.into(),

            src,
        }
    }
}
