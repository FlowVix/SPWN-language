use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Range;
use std::path::PathBuf;
use std::rc::Rc;

use ahash::AHashMap;

use crate::bytecode::Bytecode;
use crate::util::slabmap::SlabMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display(fmt = "{}..{}", start, end)]
pub struct CodeSpan {
    pub start: usize,
    pub end: usize,
    pub source_id: usize,
}
impl Debug for CodeSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl CodeSpan {
    pub const ZEROSPAN: Self = CodeSpan {
        start: 0,
        end: 0,
        source_id: 0,
    };

    pub fn extended(self, other: Self) -> Self {
        assert_eq!(
            self.source_id, other.source_id,
            "BUG: cannot extend span of a different source"
        );
        Self {
            start: self.start,
            end: other.end,
            source_id: self.source_id,
        }
    }
}

impl From<CodeSpan> for Range<usize> {
    fn from(value: CodeSpan) -> Self {
        value.start..value.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
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

pub type BytecodeMap = AHashMap<&'static SpwnSource, Bytecode>;

pub struct Source {
    pub source: SpwnSource,
    pub id: usize,
    // pub start_offset: usize,
    // pub len: usize,
    pub name: String,
    pub src: Option<String>,
}

impl Source {
    pub fn read_to_string(&self) -> Option<String> {
        self.source.read()
    }
}

#[derive(Default)]
struct SourceMapFiles {
    sources: Vec<Rc<Source>>,
}

#[derive(Default)]
pub struct SourceMap {
    files: Rc<RefCell<SourceMapFiles>>,
}

impl SourceMap {
    pub fn new_source_file(&self, source: SpwnSource) -> Option<Rc<Source>> {
        let mut files = self.files.borrow_mut();

        let src = source.read()?;

        let current_id = files.sources.len();

        let source = Rc::new(Source {
            name: source.name(),
            source,
            id: current_id,
            src: Some(src),
        });

        files.sources.push(Rc::clone(&source));

        Some(source)
    }

    pub fn get_file_by_id(&self, id: usize) -> Option<Rc<Source>> {
        self.files.borrow().sources.get(id).map(Rc::clone)
    }
}

impl Clone for SourceMap {
    fn clone(&self) -> Self {
        Self {
            files: Rc::clone(&self.files),
        }
    }
}
