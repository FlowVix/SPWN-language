#![allow(unused_variables)]

use std::collections::HashMap;

use crate::compilation::code::{Code, ConstID, InstrNum, KeysID, MacroBuildID, VarID};
use crate::leveldata::gd_types::Id;
use crate::leveldata::object_data::{GdObj, ObjParam, ObjectMode};
use crate::parsing::ast::IdClass;
use crate::sources::CodeSpan;
use crate::vm::context::SkipMode;
use crate::vm::interpreter::ValueKey;
use crate::vm::value::{Argument, Macro, Pattern};

use super::context::{FullContext, ReturnType};
use super::error::RuntimeError;
use super::interpreter::{run_func, Globals};
use super::value::{value_ops, Value};

macro_rules! run_helper {
    ($context:ident, $globals:ident, $data:ident) => {
        #[allow(unused_macros)]
        #[allow(unused_macros)]
        macro_rules! pop {
            (Key) => {
                $context.inner().stack.pop().unwrap()
            };
            (Ref) => {
                &$globals.memory[$context.inner().stack.pop().unwrap()]
            };
            (Shallow) => {
                $globals.memory[$context.inner().stack.pop().unwrap()].clone()
            };
            (Shallow Store) => {
                $globals
                    .memory
                    .insert($globals.memory[$context.inner().stack.pop().unwrap()].clone())
            };
            (Deep) => {{
                let val = $globals.memory[$context.inner().stack.pop().unwrap()].clone();
                val.deep_clone($globals)
            }};
            (Deep Store) => {{
                $globals.key_deep_clone($context.inner().stack.pop().unwrap())
            }};
        }

        #[allow(unused_macros)]
        macro_rules! push {
            (Value: $v:expr) => {{
                let key = $globals.memory.insert($v);
                $context.inner().stack.push(key);
            }};
            (Key: $v:expr) => {{
                $context.inner().stack.push($v);
            }};
        }

        #[allow(unused_macros)]
        macro_rules! store {
            ($v:expr) => {
                $globals.memory.insert($v)
            };
        }

        #[allow(unused_macros)]
        macro_rules! area {
            () => {
                $data.code.source.area($data.span)
            };
        }
    };
}

// data passedd into an instruction function
pub struct InstrData<'a> {
    pub code: &'a Code,
    pub span: CodeSpan,
}

pub fn run_load_const(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    id: ConstID,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    push!(Value: data.code.const_register[id].to_value().into_stored(area!()));
    Ok(())
}
use paste::paste;
macro_rules! op_helper {
    ($($op_fn:ident),*) => {
        $(
            paste! {
                pub fn [<run_ $op_fn>](
                    globals: &mut Globals,
                    data: &InstrData,
                    context: &mut FullContext,
                ) -> Result<(), RuntimeError> {
                    run_helper!(context, globals, data);
                    let b = pop!(Ref);
                    let a = pop!(Ref);
                    let result = value_ops::$op_fn(a, b, area!(), globals)?;
                    push!(Value: result);
                    Ok(())
                }
            }
        )*
    };
}

op_helper! { plus, minus, mult, div, modulo, pow, eq, neq, gt, gte, lt, lte }

pub fn run_negate(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let v = pop!(Ref);
    let result = value_ops::unary_negate(v, area!())?;
    push!(Value: result);
    Ok(())
}

pub fn run_not(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let v = pop!(Ref);
    let result = value_ops::unary_not(v, area!())?;
    push!(Value: result);
    Ok(())
}

pub fn run_load_var(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    id: VarID,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    match *context.inner().vars[id.0 as usize].vec.last().unwrap() {
        Some(k) => push!(Key: k),
        None => return Err(RuntimeError::UndefinedVariable { area: area!() }),
    }
    Ok(())
}

pub fn run_set_var(
    globals: &mut Globals,
    _data: &InstrData,
    context: &mut FullContext,
    id: VarID,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    // let value = pop!(Shallow);
    match *context.inner().vars[id.0 as usize].vec.last().unwrap() {
        Some(k) => {
            let value = pop!(Deep);
            globals.memory[k] = value;
        }
        None => {
            *context.inner().vars[id.0 as usize].vec.last_mut().unwrap() = Some(pop!(Deep Store));
        }
    }
    Ok(())
}

pub fn run_create_var(
    globals: &mut Globals,
    _data: &InstrData,
    context: &mut FullContext,
    id: VarID,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);

    if globals.undefined_captured.contains(&id) {
        let k = context.inner().vars[id.0 as usize]
            .vec
            .last()
            .unwrap()
            .unwrap();
        let value = pop!(Deep);
        globals.memory[k] = value;
        globals.undefined_captured.remove(&id);
    } else {
        *context.inner().vars[id.0 as usize].vec.last_mut().unwrap() = Some(pop!(Deep Store));
    }

    Ok(())
}

pub fn run_build_array(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    len: InstrNum,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let mut items = vec![];
    for _ in 0..len.0 {
        items.push(pop!(Deep Store));
    }
    items.reverse();
    push!(Value: Value::Array(items).into_stored(data.code.source.area(data.span)));
    Ok(())
}

pub fn run_build_dict(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    keys_id: KeysID,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let key_data = &data.code.keys_register[keys_id];
    let map = key_data
        .iter()
        .map(|s| (s.clone(), pop!(Deep Store)))
        .collect();
    push!(Value: Value::Dict(map).into_stored(data.code.source.area(data.span)));
    Ok(())
}

pub fn run_jump(
    _globals: &mut Globals,
    _data: &InstrData,
    context: &mut FullContext,
    pos: InstrNum,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    context.inner().pos = pos.0 as isize - 1;
    Ok(())
}

pub fn run_jump_if_false(
    globals: &mut Globals,
    _data: &InstrData,
    context: &mut FullContext,
    pos: InstrNum,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let v = &globals.memory[pop!(Key)];
    if !value_ops::to_bool(v)? {
        context.inner().pos = pos.0 as isize - 1;
    }
    Ok(())
}

pub fn run_pop_top(
    _globals: &mut Globals,
    _data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    pop!(Key);
    Ok(())
}

pub fn run_push_empty(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    push!(Value: Value::Empty.into_stored(area!()));
    Ok(())
}

pub fn run_wrap_maybe(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let top = pop!(Deep Store);
    push!(Value: Value::Maybe(Some(top)).into_stored(area!()));
    Ok(())
}

pub fn run_push_none(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    push!(Value: Value::Maybe(None).into_stored(area!()));
    Ok(())
}

pub fn run_trigger_func_call(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn run_push_trigger_fn(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn run_print(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    println!(
        "{}    context: {}",
        ansi_term::Color::Green
            .bold()
            .paint(pop!(Ref).value.to_str(globals)),
        ansi_term::Color::Blue
            .bold()
            .paint(context.inner().group.to_str())
    );
    Ok(())
}

pub fn run_to_iter(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn run_iter_next(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    pos: InstrNum,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn run_build_macro(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    build: MacroBuildID,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);

    let (func_id, arg_info) = data.code.macro_build_register[build].clone();
    let mut args = vec![];
    let ret_pattern = pop!(Deep Store);

    for (name, has_type, has_default) in arg_info.iter().rev() {
        let default = if *has_default {
            Some(pop!(Deep Store))
        } else {
            None
        };
        let pattern = if *has_type {
            Some(pop!(Deep Store))
        } else {
            None
        };
        args.push(Argument {
            name: name.clone(),
            default,
            pattern,
        })
    }
    args.reverse();

    let mut captured = HashMap::new();
    for i in &data.code.funcs[func_id].capture_ids {
        match context.inner().vars[i.0 as usize].vec.last_mut().unwrap() {
            Some(k) => {
                captured.insert(*i, *k);
            }
            var @ None => {
                globals.undefined_captured.insert(*i);
                let k = store!(Value::Empty.into_stored(area!()));
                *var = Some(k);
                captured.insert(*i, k);
            }
        }
    }

    push!(Value: Value::Macro(Macro {
        func_id,
        captured,
        args,
        ret_pattern
    })
    .into_stored(area!()));
    Ok(())
}

pub fn run_push_any_pattern(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    push!(Value: Value::Pattern(Pattern::Any).into_stored(area!()));
    Ok(())
}

pub fn run_impl(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    keys_id: KeysID,
) -> Result<(), RuntimeError> {
    todo!()
}

pub fn run_call(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    passed_args: InstrNum,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);

    let v = pop!(Shallow);
    match &v.value {
        Value::Macro(m) => {
            let idx = m.func_id;

            for i in &data.code.funcs[idx].inner_ids {
                context.inner().vars[i.0 as usize].vec.push(None)
            }

            let passed_args = passed_args.0 as usize;
            if passed_args > m.args.len() {
                return Err(RuntimeError::TooManyArguments {
                    expected: m.args.len(),
                    provided: passed_args,
                    call_area: area!(),
                    func_area: v.def_area.clone(),
                });
            }
            let mut arg_values = vec![None; m.args.len()];

            // set defaults
            for (i, arg) in m.args.iter().enumerate() {
                if let Some(default) = arg.default {
                    arg_values[i] = Some(default)
                }
            }

            // set positional
            for i in 0..passed_args {
                let val = pop!(Deep Store);
                arg_values[m.args.len() - 1 - i] = Some(val);
            }

            // apply
            for (i, var_id) in data.code.funcs[idx].arg_ids.iter().enumerate() {
                // set variable
                let val = match arg_values[i] {
                    Some(v) => v,
                    None => {
                        return Err(RuntimeError::ArgumentNotSatisfied {
                            arg_name: todo!(),
                            call_area: area!(),
                            arg_area: todo!(),
                        })
                    }
                };
                *context.inner().vars[var_id.0 as usize]
                    .vec
                    .last_mut()
                    .unwrap() = Some(val);
            }

            let stored_pos = context.inner().pos;

            for (id, k) in &m.captured {
                *context.inner().vars[id.0 as usize].vec.last_mut().unwrap() = Some(*k);
            }

            run_func(globals, data.code, idx, context)?;

            for context in context.iter(SkipMode::IncludeReturns) {
                for i in &data.code.funcs[idx].inner_ids {
                    context.inner().vars[i.0 as usize].vec.pop();
                }
                match context.inner().returned {
                    Some(ReturnType::Explicit(v)) => {
                        context.inner().stack.push(v);
                        context.inner().returned = None;
                        context.inner().pos = stored_pos;
                    }
                    Some(ReturnType::Implicit) => {
                        context
                            .inner()
                            .stack
                            .push(globals.memory.insert(Value::Empty.into_stored(area!())));
                        context.inner().returned = None;
                        context.inner().pos = stored_pos;
                    }
                    _ => unreachable!(),
                }
            }
        }
        _ => {
            return Err(RuntimeError::CannotCall {
                base: v.clone(),
                area: v.def_area.clone(),
            })
        }
    }
    Ok(())
}

pub fn run_return(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    let val = pop!(Deep Store); // Key?
    context.inner().returned = Some(ReturnType::Explicit(val));
    Ok(())
}

pub fn run_index(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);

    let index = pop!(Shallow);
    let base = pop!(Shallow);
    match base.value {
        Value::Array(arr) => match index.value {
            Value::Int(n) => push!(Key: arr[n as usize]),
            _ => panic!("fuck uu"),
        },
        _ => panic!("fuck u"),
    }
    Ok(())
}

pub fn run_yeet_context(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    context.inner().yeeted = true;
    Ok(())
}

pub fn run_enter_arrow_statement(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    end_pos: InstrNum,
) -> Result<(), RuntimeError> {
    // split context

    let mut outside_context = context.inner().clone();

    // send one context to the end
    outside_context.pos = (end_pos.0 - 1) as isize;

    *context = FullContext::Split(
        Box::new(context.clone()),
        Box::new(FullContext::Single(outside_context)),
    );

    Ok(())
}

pub fn run_enter_trigger_function(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    end_pos: InstrNum,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);
    // get new arbitrary group and stuff
    let trig_fn_group = Id::Arbitrary(globals.arbitrary_ids[IdClass::Group as usize]);
    globals.arbitrary_ids[IdClass::Group as usize] += 1;

    // split context
    let mut outside_context = context.inner().clone();
    let mut inside_context = context.inner().clone();

    // send one context to the end
    outside_context.pos = (end_pos.0 - 1) as isize;

    outside_context.stack.push(store!(Value::TriggerFn {
        start_group: trig_fn_group,
    }
    .into_stored(area!())));

    // setup inside context
    inside_context.group = trig_fn_group;

    *context = FullContext::Split(
        Box::new(FullContext::Single(inside_context)),
        Box::new(FullContext::Single(outside_context)),
    );

    Ok(())
}

fn build_object(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    n: InstrNum,
    mode: ObjectMode,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);

    let mut obj = GdObj {
        params: HashMap::new(),
        mode,
    };
    for _ in 0..n.0 {
        let val = pop!(Deep);
        let key = pop!(Shallow);
        // make sure key is number (for now)
        let key = match key.value {
            Value::Int(n) => n as u16,
            _ => {
                return Err(RuntimeError::TypeMismatch {
                    v: key.clone(),
                    expected: "integer".to_string(),
                    area: key.def_area,
                })
            }
        };
        // convert to obj param
        let param = match val.value {
            Value::Int(n) => ObjParam::Number(n as f64),
            Value::Float(x) => ObjParam::Number(x),
            Value::String(s) => ObjParam::Text(s),
            Value::Bool(b) => ObjParam::Bool(b),
            Value::Group(g) => ObjParam::Group(g),
            Value::TriggerFn { start_group } => ObjParam::Group(start_group),
            _ => {
                return Err(RuntimeError::TypeMismatch {
                    v: val.clone(),
                    expected: "valid object property value".to_string(),
                    area: val.def_area,
                })
            }
        };
        obj.params.insert(key, param);
    }
    push!(Value: Value::Object(obj).into_stored(area!()));
    Ok(())
}

pub fn run_build_trigger(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    n: InstrNum,
) -> Result<(), RuntimeError> {
    build_object(globals, data, context, n, ObjectMode::Trigger)
}

pub fn run_build_object(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
    n: InstrNum,
) -> Result<(), RuntimeError> {
    build_object(globals, data, context, n, ObjectMode::Object)
}

pub fn run_add_object(
    globals: &mut Globals,
    data: &InstrData,
    context: &mut FullContext,
) -> Result<(), RuntimeError> {
    run_helper!(context, globals, data);

    let object = pop!(Shallow);
    match object.value {
        Value::Object(mut obj) => match obj.mode {
            ObjectMode::Object => {
                if context.inner().group != Id::Specific(0) {
                    return Err(RuntimeError::AddObjectAtRuntime { area: area!() });
                }
                globals.objects.push(obj)
            }
            ObjectMode::Trigger => {
                obj.params
                    .insert(57, ObjParam::Group(context.inner().group));
                globals.triggers.push(obj)
            }
        },
        _ => {
            return Err(RuntimeError::TypeMismatch {
                v: object.clone(),
                expected: "obj or trigger".to_string(),
                area: object.def_area,
            })
        }
    };
    Ok(())
}
