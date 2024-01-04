use super::error::RuntimeError;
use super::memory::{Memory, ValueKey};
use super::value::{Value, ValueType};
use super::{RunInfo, RuntimeResult};
use crate::parser::operators::BinOp;
use crate::source::CodeArea;

pub fn to_bool(
    memory: &Memory,
    v: ValueKey,
    area: CodeArea,
    // run_info: RunInfo,
) -> RuntimeResult<bool> {
    Ok(match &memory[v].value {
        Value::Bool(v) => *v,
        _ => {
            return Err(RuntimeError::TypeMismatch {
                value: (memory[v].value.get_type(), memory[v].def_area),
                expected: &[ValueType::Bool],
                area,
            })
        },
    })
}

pub fn plus(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::Plus,
                area,
            })
        },
    })
}
pub fn minus(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::Minus,
                area,
            })
        },
    })
}
pub fn mult(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::Mult,
                area,
            })
        },
    })
}
pub fn div(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::Div,
                area,
            })
        },
    })
}

pub fn gt(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::Gt,
                area,
            })
        },
    })
}
pub fn gte(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::GtE,
                area,
            })
        },
    })
}
pub fn lt(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::Lt,
                area,
            })
        },
    })
}
pub fn lte(
    memory: &mut Memory,
    a: ValueKey,
    b: ValueKey,
    area: CodeArea,
    run_info: RunInfo,
) -> RuntimeResult<Value> {
    Ok(match (&memory[a].value, &memory[b].value) {
        (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
        (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
        _ => {
            return Err(RuntimeError::InvalidOperands {
                v1: (memory[a].value.get_type(), memory[a].def_area),
                v2: (memory[b].value.get_type(), memory[b].def_area),
                op: BinOp::LtE,
                area,
            })
        },
    })
}
