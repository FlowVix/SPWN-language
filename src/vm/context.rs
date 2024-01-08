use std::cmp::Ordering;
use std::mem::ManuallyDrop;

use binary_heap_plus::{BinaryHeap, FnComparator, PeekMut};
use itertools::Itertools;

use super::memory::{Memory, StoredValue, ValueKey};
use super::value::Value;
use super::RunInfo;
use crate::util::slabmap::SlabMap;

#[derive(Debug, Clone)]
pub struct FuncStackItem {
    pub vars: Vec<Option<ValueKey>>,
    pub stack: Vec<ValueKey>,
}

#[derive(Debug)]
pub struct Context {
    pub memory: Memory,

    pub ip: usize,
    pub func_stack: Vec<FuncStackItem>,

    pub id: usize,
}

static mut CONTEXT_ID_COUNT: usize = 0;
fn next_ctx_id() -> usize {
    unsafe {
        CONTEXT_ID_COUNT += 1;
        CONTEXT_ID_COUNT - 1
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            memory: self.memory.clone(),
            ip: self.ip,
            func_stack: self.func_stack.clone(),
            id: next_ctx_id(),
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            memory: SlabMap::new(),
            ip: 0,
            func_stack: vec![],
            id: next_ctx_id(),
        }
    }

    #[inline]
    pub fn stack(&self) -> &Vec<ValueKey> {
        &self.func_stack.last().unwrap().stack
    }

    #[inline]
    pub fn stack_mut(&mut self) -> &mut Vec<ValueKey> {
        &mut self.func_stack.last_mut().unwrap().stack
    }

    #[inline]
    pub fn stack_pop(&mut self) -> ValueKey {
        self.stack_mut().pop().unwrap()
    }

    #[inline]
    pub fn stack_push(&mut self, k: ValueKey) {
        self.stack_mut().push(k)
    }

    #[inline]
    pub fn vars(&self) -> &Vec<Option<ValueKey>> {
        &self.func_stack.last().unwrap().vars
    }

    #[inline]
    pub fn vars_mut(&mut self) -> &mut Vec<Option<ValueKey>> {
        &mut self.func_stack.last_mut().unwrap().vars
    }

    pub fn value_display(&self, v: &Value) -> String {
        match v {
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Bool(v) => v.to_string(),
            Value::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|k| self.value_display(&self.memory[*k].value))
                    .join(", ")
            ),
            Value::Empty => "()".into(),
            Value::Macro { .. } => "macro todo".into(),
        }
    }
}

pub trait DeepClone<I> {
    fn memory(&mut self) -> &mut Memory;

    fn deep_clone(&mut self, input: I) -> StoredValue;
    fn deep_clone_key(&mut self, input: I) -> ValueKey {
        let v = self.deep_clone(input);
        self.memory().insert(v)
    }
}

impl DeepClone<ValueKey> for Context {
    #[inline]
    fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    #[inline]
    fn deep_clone(&mut self, input: ValueKey) -> StoredValue {
        let stored: ManuallyDrop<StoredValue> =
            unsafe { ManuallyDrop::new(std::ptr::read(&self.memory[input] as *const _)) };
        match &stored.value {
            Value::Array(arr) => Value::Array(
                arr.iter()
                    .map(|k| {
                        let v = self.deep_clone(*k);
                        self.memory.insert(v)
                    })
                    .collect(),
            ),
            v => v.clone(),
        }
        .into_stored(stored.def_area)
    }
}
impl DeepClone<&StoredValue> for Context {
    #[inline]
    fn memory(&mut self) -> &mut Memory {
        &mut self.memory
    }

    #[inline]
    fn deep_clone(&mut self, input: &StoredValue) -> StoredValue {
        match &input.value {
            Value::Array(arr) => Value::Array(
                arr.iter()
                    .map(|k| {
                        let v = self.deep_clone(*k);
                        self.memory.insert(v)
                    })
                    .collect(),
            ),
            v => v.clone(),
        }
        .into_stored(input.def_area)
    }
}

type CtxComparator = FnComparator<fn(&Context, &Context) -> Ordering>;
#[inline]
fn ip_cmp_rev(a: &Context, b: &Context) -> Ordering {
    a.ip.cmp(&b.ip).reverse()
}

pub struct FullContext<'a> {
    pub contexts: BinaryHeap<Context, CtxComparator>,

    pub run_info: RunInfo<'a>,
    pub have_returned: bool,
}

impl<'a> FullContext<'a> {
    pub fn new(initial: Context, run_info: RunInfo<'a>) -> Self {
        let mut contexts: BinaryHeap<Context, CtxComparator> = BinaryHeap::new_by(ip_cmp_rev);

        contexts.push(initial);
        Self {
            contexts,
            have_returned: false,
            run_info,
        }
    }

    pub fn current(&self) -> &Context {
        self.contexts.peek().expect("BUG: no current context")
    }

    pub fn current_mut(&mut self) -> PeekMut<Context, CtxComparator> {
        self.contexts.peek_mut().expect("BUG: no current context")
    }

    pub fn valid(&self) -> bool {
        !self.contexts.is_empty()
    }

    pub fn yeet_current(&mut self) -> Option<Context> {
        self.contexts.pop()
    }
}
