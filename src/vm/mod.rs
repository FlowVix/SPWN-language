use std::fmt::Debug;
use std::rc::Rc;

use colored::Colorize;

use self::context::Context;
use self::error::RuntimeError;
use self::memory::{MemKey, Memory};
use self::value::{StoredValue, Value};
use crate::bytecode::opcodes::Opcode;
use crate::bytecode::{Bytecode, Function};
use crate::source::{CodeArea, CodeSpan, SpwnSource};
use crate::util::ImmutVec;
use crate::vm::context::FullContext;

pub mod context;
pub mod error;
pub mod memory;
pub mod multi;
pub mod value;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Clone, Copy)]
pub struct RunInfo {
    pub program: &'static ImmutVec<Bytecode>,
    pub bytecode_idx: usize,
    pub func_idx: usize,
}

impl Debug for RunInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<code: {}, func: {}>", self.bytecode_idx, self.func_idx)
    }
}

impl RunInfo {
    #[inline]
    pub fn bytecode(&self) -> &Bytecode {
        &self.program[self.bytecode_idx]
    }

    #[inline]
    pub fn func(&self) -> &Function {
        &self.bytecode().funcs[self.func_idx]
    }

    #[inline]
    pub fn opcodes(&self) -> &[Opcode] {
        &self.func().opcodes
    }

    #[inline]
    pub fn src(&self) -> &Rc<SpwnSource> {
        &self.bytecode().src
    }

    #[inline]
    pub fn consts(&self) -> &[Value] {
        &self.bytecode().consts
    }

    #[inline]
    pub fn make_area(&self, span: CodeSpan) -> CodeArea {
        CodeArea {
            span,
            src: self.src().clone(),
        }
    }
}

pub struct Vm {
    pub memory: Memory,
}

impl Vm {
    #[inline]
    pub fn insert(&mut self, v: StoredValue) -> MemKey {
        self.memory.insert(v)
    }

    #[inline]
    pub fn get(&self, k: MemKey) -> &StoredValue {
        &self.memory[k]
    }

    #[inline]
    pub fn get_mut(&mut self, k: MemKey) -> &mut StoredValue {
        &mut self.memory[k]
    }

    /// <img src="https://cdna.artstation.com/p/assets/images/images/056/833/046/original/lara-hughes-blahaj-spin-compressed.gif?1670214805" width=60 height=60>
    /// <img src="https://cdna.artstation.com/p/assets/images/images/056/833/046/original/lara-hughes-blahaj-spin-compressed.gif?1670214805" width=60 height=60>
    /// <img src="https://cdna.artstation.com/p/assets/images/images/056/833/046/original/lara-hughes-blahaj-spin-compressed.gif?1670214805" width=60 height=60>
    pub fn run_func<F>(&mut self, mut context: Context, info: RunInfo) -> Vec<Context> {
        let original_ip = context.ip;
        context.ip = 0;

        let mut full_ctx = FullContext::new(context, info);

        let mut out_contexts = vec![];

        while full_ctx.valid() {
            let ip = full_ctx.current().ip;

            if ip > info.opcodes().len() {
                if !full_ctx.have_returned {
                    let mut top = full_ctx.yeet_current().unwrap();
                    top.func_stack.pop();

                    out_contexts.push(top);
                } else {
                    full_ctx.yeet_current();
                }
                continue;
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum LoopFlow {
                Normal,
                Continue,
            }

            let run_opcode = |opcode: Opcode, opcode_span: CodeSpan| -> RuntimeResult<LoopFlow> {
                // match opcode {
                //     Opcode::PushConst(id) => full_ctx.current_mut().stack.push(
                //         info.consts()[*id as usize]
                //             .clone()
                //             .into_stored(info.make_area(opcode_span), false),
                //     ),
                //     Opcode::PopTop => {
                //         full_ctx.current_mut().stack.pop();
                //     },
                //     Opcode::Plus => todo!(),
                //     Opcode::Minus => todo!(),
                //     Opcode::Mult => todo!(),
                //     Opcode::Div => todo!(),
                //     Opcode::Modulo => todo!(),
                //     Opcode::Pow => todo!(),
                //     Opcode::Dbg => {
                //         let v = full_ctx.current_mut().pop();
                //         println!("{}", format!("{:?}", v.value).bright_green());
                //     },
                // }

                todo!()
            };
        }

        todo!()
    }
}
