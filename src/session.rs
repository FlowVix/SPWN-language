use std::cell::RefCell;

use ahash::RandomState;
use lasso::{Rodeo, Spur};

use crate::errors::DiagCtx;
use crate::source::{BytecodeMap, SourceMap, SpwnSource};
use crate::util::interner::Interner;

// maybe in the future we will need a separate session for the parser?
#[non_exhaustive]
pub struct Session {
    pub input: &'static SpwnSource,
    pub diag_ctx: DiagCtx,
    pub interner: RefCell<Rodeo<Spur, RandomState>>,
    pub spwn_version: &'static str,
    pub trailing_args: Vec<String>,
    pub bytecode_map: BytecodeMap,
    pub source_map: SourceMap,
    // cli args, allowed features, deprecated features ....
}

impl Session {
    pub fn new_standard(input: SpwnSource, trailing_args: Vec<String>) -> Self {
        let source_map = SourceMap::default();
        Self::new_custom(
            source_map.clone(),
            DiagCtx::with_standard_emitter(source_map),
            input,
            trailing_args,
        )
    }

    pub fn new_custom(
        source_map: SourceMap,
        diag_ctx: DiagCtx,
        input: SpwnSource,
        trailing_args: Vec<String>,
    ) -> Self {
        Self {
            input: Box::leak(Box::new(input)),
            diag_ctx,
            interner: RefCell::new(Rodeo::with_hasher(RandomState::new())),
            spwn_version: env!("CARGO_PKG_VERSION"),
            trailing_args,
            bytecode_map: BytecodeMap::new(),
            source_map,
        }
    }

    pub fn source_code(&self) -> Option<String> {
        self.input.read()
    }
}
