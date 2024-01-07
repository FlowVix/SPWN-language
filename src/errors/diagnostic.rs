use std::panic;

use ahash::AHashSet;
use colored::{Color, Style};

use super::{Emitter, StandardEmitter};
use crate::source::CodeArea;

pub struct ErrorGuaranteed;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    Error,
    Warning,
}

impl From<Level> for Color {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => Color::TrueColor {
                r: 255,
                g: 72,
                b: 72,
            },
            Level::Warning => Color::TrueColor {
                r: 252,
                g: 255,
                b: 72,
            },
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Diagnostic {
    pub(crate) level: Level,
    pub title: String,
    pub message: String,
    pub labels: Vec<(String, CodeArea)>,
    pub suggestions: Vec<String>,
}

impl Diagnostic {
    pub fn new<T, M>(level: Level, title: T, message: M) -> Self
    where
        T: Into<String>,
        M: Into<String>,
    {
        Self {
            level,
            title: title.into(),
            message: message.into(),
            labels: vec![],
            suggestions: vec![],
        }
    }
}

#[non_exhaustive]
pub struct DiagCtx {
    pub(crate) emitter: Box<dyn Emitter>,
}

impl DiagCtx {
    pub fn with_standard_emitter() -> Self {
        Self::with_emitter(StandardEmitter)
    }

    pub fn with_emitter(emitter: impl Emitter + 'static) -> Self {
        DiagCtx {
            emitter: Box::new(emitter),
        }
    }

    pub fn emit_error(&mut self, error: impl Into<Diagnostic>) -> ErrorGuaranteed {
        self.emitter
            .emit(&error.into())
            .expect("BUG: failed to emit error");
        ErrorGuaranteed
    }

    pub fn emit_warning(&mut self, warning: impl Into<Diagnostic>) {
        self.emitter
            .emit(&warning.into())
            .expect("BUG: failed to emit warning")
    }
}
