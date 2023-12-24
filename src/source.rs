use std::fmt::Debug;
use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;

use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Display)]
#[display(fmt = "{}..{}", start, end)]
pub struct CodeSpan {
    pub start: usize,
    pub end: usize,
}

impl CodeSpan {
    pub const ZEROSPAN: Self = CodeSpan { start: 0, end: 0 };

    pub fn extended(self, other: Self) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }

    pub fn into_area(self, src: Rc<SpwnSource>) -> CodeArea {
        CodeArea { span: self, src }
    }
}

impl From<Range<usize>> for CodeSpan {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
impl From<CodeSpan> for Range<usize> {
    fn from(value: CodeSpan) -> Self {
        value.start..value.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpwnSource {
    File(PathBuf),
}

impl SpwnSource {
    pub fn read(&self) -> Option<String> {
        match self {
            SpwnSource::File(path) => std::fs::read_to_string(path).ok(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            SpwnSource::File(path) => path.to_str().unwrap().into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Display)]
#[display(fmt = "<{} @ {}>", "src.name()", span)]
pub struct CodeArea {
    pub span: CodeSpan,
    pub src: Rc<SpwnSource>,
}
