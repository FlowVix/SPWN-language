use std::collections::BTreeMap;

use super::memory::MemKey;
use crate::source::CodeArea;
use crate::util::String32;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String32),
    Bool(bool),

    Array(Vec<MemKey>),
    Dict(BTreeMap<MemKey, MemKey>),

    Maybe(Option<Box<Value>>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoredValue {
    pub value: Value,
    pub def_area: CodeArea,
}

impl Value {
    pub fn into_stored(self, def_area: CodeArea) -> StoredValue {
        StoredValue {
            value: self,
            def_area,
        }
    }
}
