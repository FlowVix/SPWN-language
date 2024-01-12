use std::borrow::BorrowMut;
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
    diag_ctx: &'a DiagCtx,

    pub(crate) diagnostic: Box<Diagnostic>,
}

impl<'a> Debug for DiagnosticBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.diagnostic)
    }
}

impl<'a> Deref for DiagnosticBuilder<'a> {
    type Target = Diagnostic;

    fn deref(&self) -> &Self::Target {
        self.diagnostic.as_ref()
    }
}
impl<'a> DerefMut for DiagnosticBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.diagnostic.as_mut()
    }
}

impl<'a> DiagnosticBuilder<'a> {
    pub fn emit(self) -> ErrorGuaranteed {
        self.diag_ctx.emit_error(*self.diagnostic)
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

pub struct DiagCtx {
    inner: RefCell<DiagCtxInner>,
}

struct DiagCtxInner {
    pub(crate) emitter: Box<dyn Emitter>,
    error_count: usize,
}

impl DiagCtx {
    pub fn with_standard_emitter(src_map: SourceMap) -> Self {
        Self::with_emitter(StandardEmitter::new(src_map))
    }

    pub fn with_emitter(emitter: impl Emitter + 'static) -> Self {
        Self {
            inner: RefCell::new(DiagCtxInner {
                emitter: Box::new(emitter),
                error_count: 0,
            }),
        }
    }

    pub fn create_error(&self, diagnostic: impl Into<Diagnostic>) -> DiagnosticBuilder<'_> {
        DiagnosticBuilder {
            diag_ctx: self,
            diagnostic: Box::new(diagnostic.into()),
        }
    }

    pub fn emit_error(&self, error: impl Into<Diagnostic>) -> ErrorGuaranteed {
        let mut slf = self.inner.borrow_mut();
        slf.error_count += 1;
        slf.emitter
            .emit(&error.into())
            .expect("BUG: failed to emit error");
        ErrorGuaranteed
    }

    pub fn emit_warning(&self, warning: impl Into<Diagnostic>) {
        self.inner
            .borrow_mut()
            .emitter
            .emit(&warning.into())
            .expect("BUG: failed to emit warning")
    }

    #[inline(always)]
    pub fn error_count(&self) -> usize {
        self.inner.borrow().error_count
    }

    #[inline(always)]
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    pub fn abort_if_errors(&self) -> Option<ErrorGuaranteed> {
        self.has_errors().then_some(ErrorGuaranteed)
    }
}
