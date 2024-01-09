use super::value::Value;
use crate::source::CodeSpan;
use crate::util::slabmap::SlabMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Into, derive_more::From)]
pub struct ValueKey(usize);

#[derive(Debug, Clone)]
pub struct StoredValue {
    pub value: Value,
    pub def_span: CodeSpan,
}

pub type Memory = SlabMap<ValueKey, StoredValue>;
