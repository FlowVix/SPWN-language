use std::cmp::Ordering;
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

use derive_more::{Deref, DerefMut};

use super::memory::MemKey;
use super::RunInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct FuncStackItem {
    //
}

/// <h1> HDMIEFHMSMSMAMAMAM GET MOLEY ON YOUR PHONEI AM GOING TO EAT this LITTLE CHINESE BOY!! </h1>
/// I am going to Eat this LIttle CHINESE BOY how old are you 18 LOUDER 18 I have have him absolutely giftwrapped in my signature details single button narrow peak lapelle
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub stack: Vec<MemKey>,
    pub ip: usize,

    pub func_stack: Vec<FuncStackItem>,
}

impl Context {
    pub fn pop(&mut self) -> MemKey {
        self.stack.pop().unwrap()
    }
}

impl Eq for Context {}

impl PartialOrd for Context {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Context {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ip.cmp(&other.ip).reverse()
    }
}

#[derive(Debug)]
pub struct FullContext {
    pub contexts: BinaryHeap<Context>,

    pub run_info: RunInfo,
    pub have_returned: bool,
}

impl FullContext {
    pub fn new(initial: Context, run_info: RunInfo) -> Self {
        let mut contexts = BinaryHeap::new();
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

    pub fn current_mut(&mut self) -> PeekMut<Context> {
        self.contexts.peek_mut().expect("BUG: no current context")
    }

    pub fn valid(&self) -> bool {
        !self.contexts.is_empty()
    }

    pub fn yeet_current(&mut self) -> Option<Context> {
        self.contexts.pop()
    }
}

// #[derive(Debug, Deref, DerefMut)]
// pub struct ContextStack(pub Vec<FullContext>);

// impl ContextStack {
//     pub fn last(&self) -> &FullContext {
//         self.0.last().unwrap()
//     }

//     pub fn last_mut(&mut self) -> &mut FullContext {
//         self.0.last_mut().unwrap()
//     }

//     pub fn current(&self) -> &Context {
//         self.last().current()
//     }

//     pub fn current_mut(&mut self) -> PeekMut<Context> {
//         self.last_mut().current_mut()
//     }
// }
