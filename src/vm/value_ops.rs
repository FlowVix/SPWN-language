use super::error::RuntimeError;
use super::memory::{Memory, ValueKey};
use super::value::{Value, ValueType};
use super::{RunInfo, RuntimeResult};
use crate::errors::DiagCtx;
use crate::parser::operators::BinOp;
use crate::source::CodeSpan;

pub fn to_bool(
    diag_ctx: &mut DiagCtx,
    memory: &Memory,
    v: ValueKey,
    span: CodeSpan,
    // run_info: RunInfo,
) -> RuntimeResult<bool> {
    Ok(match &memory[v].value {
        Value::Bool(v) => *v,
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::TypeMismatch {
                value: (memory[v].value.get_type(), memory[v].def_span),
                expected: &[ValueType::Bool],
                span,
            }))
        },
    })
}

pub fn plus(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::Plus,
                span,
            }))
        },
    })
}
pub fn minus(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::Minus,
                span,
            }))
        },
    })
}
pub fn mult(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::Mult,
                span,
            }))
        },
    })
}
pub fn div(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::Div,
                span,
            }))
        },
    })
}

pub fn gt(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::Gt,
                span,
            }))
        },
    })
}
pub fn gte(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::GtE,
                span,
            }))
        },
    })
}
pub fn lt(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::Lt,
                span,
            }))
        },
    })
}
pub fn lte(
    diag_ctx: &mut DiagCtx,
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    span: CodeSpan,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
        _ => {
            return Err(diag_ctx.emit_error(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_span),
                v2: (memory[b].value.get_type(), memory[b].def_span),
                op: BinOp::LtE,
                span,
            }))
        },
    })
}
