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
    diagnostic: Diagnostic,
}

impl<'a> Deref for DiagnosticBuilder<'a> {
    type Target = Diagnostic;

    fn deref(&self) -> &Self::Target {
        &self.diagnostic
    }
}
impl<'a> DerefMut for DiagnosticBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.diagnostic
    }
}
impl<'a> DiagnosticBuilder<'a> {
    pub fn emit(self) -> ErrorGuaranteed {
        self.diag_ctx.emit_error(self.diagnostic)
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
    errors: Vec<Diagnostic>,
}

impl DiagCtx {
    pub fn with_standard_emitter(src_map: SourceMap) -> Self {
        Self::with_emitter(StandardEmitter::new(src_map))
    }

    pub fn with_emitter(emitter: impl Emitter + 'static) -> Self {
        DiagCtx {
            emitter: Box::new(emitter),
            errors: vec![],
        }
    }

    pub fn create_error(&mut self, diagnostic: impl Into<Diagnostic>) -> DiagnosticBuilder<'_> {
        DiagnosticBuilder {
            diag_ctx: self,
            diagnostic: diagnostic.into(),
        }
    }

    pub fn emit_error(&mut self, error: impl Into<Diagnostic>) -> ErrorGuaranteed {
        println!("gggggggggg");
        self.errors.push(error.into());
        // self.emitter
        //     .emit(&error.into())
        //     .expect("BUG: failed to emit error");
        ErrorGuaranteed
    }

    pub fn emit_warning(&mut self, warning: impl Into<Diagnostic>) {
        self.emitter
            .emit(&warning.into())
            .expect("BUG: failed to emit warning")
    }

    #[inline(always)]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    #[inline(always)]
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    pub fn abort_if_errors(&mut self) -> Option<ErrorGuaranteed> {
        println!("gla");
        for i in &self.errors {
            self.emitter.emit(i).expect("BUG: failed to emit warning");
        }
        self.has_errors().then_some(ErrorGuaranteed)
    }
}
