use ahash::RandomState;
use lasso::{Rodeo, Spur};

use crate::errors::DiagCtx;
use crate::source::{BytecodeMap, SpwnSource};
use crate::util::interner::Interner;

// maybe in the future we will need a separate session for the parser?
#[non_exhaustive]
pub struct Session {
    pub input: &'static SpwnSource,
    pub diag_ctx: DiagCtx,
    // pub interner: Interner,
    pub interner: Rodeo<Spur, RandomState>,
    pub spwn_version: &'static str,
    pub trailing_args: Vec<String>,
    pub bytecode_map: BytecodeMap,
    // cli args, allowed features, deprecated features ....
}

impl Session {
    pub fn new_standard(input: SpwnSource, trailing_args: Vec<String>) -> Self {
        Self::new_custom(DiagCtx::with_standard_emitter(), input, trailing_args)
    }

    pub fn new_custom(diag_ctx: DiagCtx, input: SpwnSource, trailing_args: Vec<String>) -> Self {
        Self {
            input: Box::leak(Box::new(input)),
            diag_ctx,
            interner: Rodeo::with_hasher(RandomState::new()),
            spwn_version: env!("CARGO_PKG_VERSION"),
            trailing_args,
            bytecode_map: BytecodeMap::new(),
        }
    }

    pub fn source_code(&self) -> Option<String> {
        self.input.read()
    }
}
