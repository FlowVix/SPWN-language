use colored::Colorize;

use self::context::{Context, FuncStackItem};
use self::error::RuntimeError;
use self::memory::ValueKey;
use self::multi::Multi;
use self::value::Value;
use crate::bytecode::constant::Constant;
use crate::bytecode::opcode::Opcode;
use crate::bytecode::{Bytecode, Function};
use crate::errors::ErrorGuaranteed;
use crate::session::Session;
use crate::source::{CodeArea, CodeSpan, SpwnSource};
use crate::util::ImmutVec;
use crate::vm::context::{DeepClone, FullContext};

pub mod context;
pub mod error;
pub mod memory;
pub mod multi;
pub mod value;
pub mod value_ops;

pub type RuntimeResult<T> = Result<T, ErrorGuaranteed>;

// #[derive(Clone, Copy)]
// pub struct RunInfo {
//     pub program: &'static ImmutVec<Bytecode>,
//     pub bytecode_idx: usize,
//     pub func_idx: usize,
// }

#[derive(Clone, Copy)]
pub struct RunInfo<'a> {
    pub bytecode: &'a Bytecode,
    pub function: &'a Function,
}

// impl std::fmt::Debug for RunInfo {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "<code: {}, func: {}>", self.bytecode_idx, self.func_idx)
//     }
// }

impl<'a> RunInfo<'a> {
    pub fn from_start(session: &'a Session) -> Self {
        let bytecode = &session.bytecode_map[&session.input];
        Self {
            bytecode,
            function: &bytecode.funcs[0],
        }
    }

    #[inline]
    pub fn opcodes(&self) -> &[(Opcode, CodeSpan)] {
        &self.function.opcodes
    }

    #[inline]
    pub fn src(&self) -> &'static SpwnSource {
        self.bytecode.src
    }

    #[inline]
    pub fn consts(&self) -> &[Constant] {
        &self.bytecode.consts
    }

    #[inline]
    pub fn make_area(&self, span: CodeSpan) -> CodeArea {
        CodeArea {
            span,
            src: self.src(),
        }
    }
}

#[non_exhaustive]
pub struct Vm<'a> {
    pub session: &'a mut Session,
}

impl<'a> Vm<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        Self { session }
    }

    // pub fn const_to_value(&self, context: &mut Context, c: &Constant) -> Value {
    //     match c {
    //         Constant::Int(v) => Value::Int(*v),
    //         Constant::Float(v) => Value::Float(*v),
    //         // Constant::Array(v) => Value::Array(v),
    //     }
    // }

    pub fn run_func(
        &mut self,
        mut context: Context,
        info: RunInfo,
    ) -> Multi<RuntimeResult<ValueKey>> {
        let original_ip = context.ip;
        context.ip = 0;

        context.func_stack.push(FuncStackItem {
            stack: vec![],
            vars: vec![None; info.function.var_count as usize],
        });

        let mut full_ctx = FullContext::new(context, info);

        let mut out = Multi::new();
        let mut out_context = |mut ctx: Context, v| {
            ctx.func_stack.pop();
            ctx.ip = original_ip;

            out.push(ctx, v);
        };

        while full_ctx.valid() {
            let ip = full_ctx.current().ip;

            if ip >= info.opcodes().len() {
                if !full_ctx.have_returned {
                    let mut top = full_ctx.yeet_current().unwrap();

                    let k = top
                        .memory
                        .insert(Value::Empty.into_stored(CodeSpan::ZEROSPAN));

                    out_context(top, Ok(k))
                } else {
                    full_ctx.yeet_current();
                }
                continue;
            }
            // {
            //     let ctx = full_ctx.current();
            //     println!("id {}, pos {}", ctx.id, ctx.ip);
            // }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            enum LoopFlow {
                Normal,
                Continue,
            }

            let mut run_opcode =
                |opcode: Opcode, opcode_span: CodeSpan| -> RuntimeResult<LoopFlow> {
                    let run_info = full_ctx.run_info;
                    //let opcode_span = run_info.make_area(opcode_span);

                    macro_rules! bin_op {
                        ($op:ident, $a:ident, $b:ident, $to:ident) => {{
                            let mut ctx = full_ctx.current_mut();
                            let b = ctx.stack_pop();
                            let a = ctx.stack_pop();
                            let v = value_ops::$op(
                                &mut self.session.diag_ctx,
                                &mut ctx.memory,
                                a,
                                b,
                                opcode_span,
                                run_info,
                            )?
                            .into_stored(opcode_span);

                            let k = ctx.memory.insert(v);

                            ctx.stack_push(k);
                        }};
                    }

                    match opcode {
                        Opcode::PopTop => {
                            full_ctx.current_mut().stack_pop();
                        },
                        Opcode::LoadConst(c) => {
                            let v = match &run_info.bytecode.consts[*c as usize] {
                                Constant::Int(v) => Value::Int(*v),
                                Constant::Float(v) => Value::Float(*v),
                                Constant::Bool(b) => Value::Bool(*b),
                                Constant::Empty => Value::Empty,
                            }
                            .into_stored(opcode_span);

                            let mut ctx = full_ctx.current_mut();
                            let k = ctx.memory.insert(v);
                            ctx.stack_push(k);
                        },
                        Opcode::Plus => bin_op!(plus, a, b, to),
                        Opcode::Minus => bin_op!(minus, a, b, to),
                        Opcode::Mult => bin_op!(mult, a, b, to),
                        Opcode::Div => bin_op!(div, a, b, to),
                        Opcode::Jump(to) => {
                            full_ctx.current_mut().ip = *to as usize;
                            return Ok(LoopFlow::Continue);
                        },
                        Opcode::JumpIfFalse(to) => {
                            let mut ctx = full_ctx.current_mut();
                            let k = ctx.stack_pop();
                            if !value_ops::to_bool(
                                &mut self.session.diag_ctx,
                                &ctx.memory,
                                k,
                                opcode_span,
                            )? {
                                ctx.ip = *to as usize;
                                return Ok(LoopFlow::Continue);
                            }
                        },
                        Opcode::JumpIfTrue(to) => {
                            let mut ctx = full_ctx.current_mut();
                            let k = ctx.stack_pop();
                            if value_ops::to_bool(
                                &mut self.session.diag_ctx,
                                &ctx.memory,
                                k,
                                opcode_span,
                            )? {
                                ctx.ip = *to as usize;
                                return Ok(LoopFlow::Continue);
                            }
                        },
                        Opcode::Mod => todo!(),
                        Opcode::Pow => todo!(),
                        Opcode::Eq => todo!(),
                        Opcode::NEq => todo!(),
                        Opcode::Gt => bin_op!(gt, a, b, to),
                        Opcode::GtE => bin_op!(gte, a, b, to),
                        Opcode::Lt => bin_op!(lt, a, b, to),
                        Opcode::LtE => bin_op!(lte, a, b, to),
                        Opcode::UnaryMinus => todo!(),
                        Opcode::UnaryNot => todo!(),
                        Opcode::MakeArray { len } => {
                            let mut v = vec![0.into(); len as usize];
                            let mut ctx = full_ctx.current_mut();

                            for i in (0..len as usize).rev() {
                                let top = ctx.stack_pop();
                                v[i] = ctx.deep_clone_key(top);
                            }
                            let k = ctx.memory.insert(Value::Array(v).into_stored(opcode_span));
                            ctx.stack_push(k);
                        },
                        Opcode::Dbg => {
                            let ctx = full_ctx.current();
                            println!(
                                "{} {}",
                                ctx.value_display(&ctx.memory[*ctx.stack().last().unwrap()].value)
                                    .bright_green(),
                                format!(":: CID {}, L {}", ctx.id, ctx.stack().len()).dimmed(),
                            )
                        },
                        Opcode::EnterArrowStatement(to) => {
                            let mut new = {
                                let mut current = full_ctx.current_mut();
                                current.ip += 1;
                                current.clone()
                            };
                            new.ip = *to as usize;
                            full_ctx.contexts.push(new);
                            return Ok(LoopFlow::Continue);
                        },
                        Opcode::YeetContext => {
                            // println!("gaga");
                            full_ctx.yeet_current();
                            return Ok(LoopFlow::Continue);
                        },
                        Opcode::SetVar(id) => {
                            let mut ctx = full_ctx.current_mut();
                            let top = ctx.stack_pop();
                            match ctx.vars_mut()[*id as usize] {
                                Some(k) => {
                                    let v = ctx.deep_clone(top);
                                    ctx.memory[k] = v;
                                },
                                None => {
                                    let k = ctx.deep_clone_key(top);
                                    ctx.vars_mut()[*id as usize] = Some(k);
                                },
                            }
                        },
                        Opcode::LoadVar(id) => {
                            let mut ctx = full_ctx.current_mut();
                            let Some(k) = ctx.vars_mut()[*id as usize] else {
                                return Err(self.session.diag_ctx.emit_error(
                                    RuntimeError::VarNotInitialized { span: opcode_span },
                                ));
                            };
                            ctx.stack_push(k)
                        },
                        Opcode::ChangeVarKey(id) => {
                            let mut ctx = full_ctx.current_mut();
                            let top = ctx.stack_pop();
                            ctx.vars_mut()[*id as usize] = Some(top);
                        },
                        Opcode::MismatchThrowIfFalse => {
                            let mut ctx = full_ctx.current_mut();
                            let top = ctx.stack_pop();
                            // TODO
                        },
                        Opcode::Return => {
                            let mut top = full_ctx.yeet_current().unwrap();
                            let k = top.stack_pop();

                            out_context(top, Ok(k));
                            return Ok(LoopFlow::Continue);
                        },
                        Opcode::MakeMacro(id) => {
                            let mut ctx = full_ctx.current_mut();
                            let k = ctx
                                .memory
                                .insert(Value::Macro { func: id }.into_stored(opcode_span));
                            ctx.stack_push(k);
                        },
                        Opcode::Call(_) => todo!(),
                    }

                    Ok(LoopFlow::Normal)
                };

            let (opcode, opcode_span) = info.opcodes()[ip];

            match run_opcode(opcode, opcode_span) {
                Ok(l) => match l {
                    LoopFlow::Normal => {
                        full_ctx.current_mut().ip += 1;
                    },
                    LoopFlow::Continue => {
                        continue;
                    },
                },
                Err(_) => {
                    // TODO: return multi with ErrorGuaranteed
                },
            }

            // run_opcode
        }

        out
    }
}
