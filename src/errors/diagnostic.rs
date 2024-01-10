use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use ahash::AHashSet;
use colored::{Color, Style};

use super::{Emitter, StandardEmitter};
use crate::source::{CodeSpan, SourceMap};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ErrorGuaranteed;

impl Debug for ErrorGuaranteed {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::fmt::Display for ErrorGuaranteed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for ErrorGuaranteed {}

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

pub struct DiagnosticBuilder<'a> {
    diag_ctx: &'a mut DiagCtx,

    diagnostic: Option<Box<Diagnostic>>,
}

impl<'a> Deref for DiagnosticBuilder<'a> {
    type Target = Diagnostic;

    fn deref(&self) -> &Self::Target {
        self.diagnostic.as_ref().unwrap()
    }
}
impl<'a> DerefMut for DiagnosticBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.diagnostic.as_mut().unwrap()
    }
}

impl<'a> Drop for DiagnosticBuilder<'a> {
    fn drop(&mut self) {
        match self.diagnostic.take() {
            Some(diag) if !std::thread::panicking() => {
                panic!(
                    "BUG: error was constructed but not emitted (error message: {})",
                    diag.message
                )
            },
            _ => (), // OK: the diagnostic was already emitted
        }
    }
}

impl<'a> DiagnosticBuilder<'a> {
    pub fn emit(mut self) -> ErrorGuaranteed {
        self.diag_ctx.emit_error(*(self.diagnostic.take().unwrap()))
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Diagnostic {
    pub(crate) level: Level,
    pub title: String,
    pub message: String,
    pub labels: Vec<(String, CodeSpan)>,
    pub notes: Vec<String>,
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
            notes: vec![],
        }
    }
}

#[non_exhaustive]
pub struct DiagCtx {
    pub(crate) emitter: Box<dyn Emitter>,
    error_count: usize,
}

impl DiagCtx {
    pub fn with_standard_emitter(src_map: SourceMap) -> Self {
        Self::with_emitter(StandardEmitter::new(src_map))
    }

    pub fn with_emitter(emitter: impl Emitter + 'static) -> Self {
        Self {
            emitter: Box::new(emitter),
            error_count: 0,
        }
    }

    pub fn create_error<'a>(
        &'a mut self,
        diagnostic: impl Into<Diagnostic>,
    ) -> DiagnosticBuilder<'a> {
        DiagnosticBuilder {
            diag_ctx: self,
            diagnostic: Some(Box::new(diagnostic.into())),
        }
    }

    pub fn emit_error(&mut self, error: impl Into<Diagnostic>) -> ErrorGuaranteed {
        self.error_count += 1;
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

    #[inline(always)]
    pub fn error_count(&self) -> usize {
        self.error_count
    }

    #[inline(always)]
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    pub fn abort_if_errors(&self) -> Option<ErrorGuaranteed> {
        self.has_errors().then_some(ErrorGuaranteed)
    }
}
