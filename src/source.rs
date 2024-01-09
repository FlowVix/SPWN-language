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

    pub fn into_area(self, src: &'static SpwnSource) -> CodeArea {
        CodeArea { span: self, src }
    }
}

// impl From<Range<usize>> for CodeSpan {
//     fn from(value: Range<usize>) -> Self {
//         Self {
//             start: value.start,
//             end: value.end,
//         }
//     }
// }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Display)]
#[display(fmt = "<{} @ {}>", "src.name()", span)]
pub struct CodeArea {
    pub span: CodeSpan,
    pub src: &'static SpwnSource,
}
impl Debug for CodeArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub type BytecodeMap = AHashMap<&'static SpwnSource, Bytecode>;

////////////

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

    // pub fn end_position(&self) -> usize {
    //     self.start_offset + self.len
    // }
}

#[derive(Default)]
struct SourceMapFiles {
    sources: Vec<Rc<Source>>,
    //id_map: AHashMap<usize, Rc<Source>>,
    last_file: Option<usize>,
}

#[derive(Default)]
pub struct SourceMap {
    files: Rc<RefCell<SourceMapFiles>>,
}

impl SourceMap {
    pub fn new_source_file(&self, source: SpwnSource) -> Option<Rc<Source>> {
        let mut files = self.files.borrow_mut();

        let src = source.read()?;

        // let last_id = if let Some(last) = files.last_file {
        //     last
        // } else {
        //     files.last_file = Some(0);
        //     0
        // };
        let current_id = files.sources.len();

        //let end_pos = files.sources[last_id].end_position();

        let source = Rc::new(Source {
            name: source.name(),
            source,
            id: current_id,
            // start_offset: end_pos + 1,
            // len: src.len(),
            src: Some(src),
        });

        files.sources.push(Rc::clone(&source));
        //files.id_map.insert(current_id, Rc::clone(&source));

        Some(source)
    }

    pub fn get_file_by_id(&self, id: usize) -> Option<Rc<Source>> {
        self.files.borrow().sources.get(id).map(Rc::clone)
    }

    // pub fn find_file_idx_by_span(&self, span: CodeSpan) -> usize {
    //     self.files
    //         .borrow()
    //         .sources
    //         .partition_point(|x| x.start_offset <= span.end)
    //         - 1
    // }

    // pub fn find_file_by_span(&self, span: CodeSpan) -> Rc<Source> {
    //     let idx = self.find_file_idx_by_span(span);
    //     (*self.files.borrow().sources)[idx].clone()
    // }
}

impl Clone for SourceMap {
    fn clone(&self) -> Self {
        Self {
            files: Rc::clone(&self.files),
        }
    }
}
