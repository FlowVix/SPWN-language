use crate::util::String32;

pub enum Constant {
    Int(i64),
    Float(f64),
    String(String32),
    Bool(bool),

    Array(Vec<Constant>),
    // Dict(BTreeMap<Value, Value>),
    Maybe(Option<Box<Constant>>),
    Empty,
}
