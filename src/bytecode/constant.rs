#[derive(Debug, Clone, PartialEq, delve::EnumDisplay)]
pub enum Constant {
    #[delve(display = |i| format!("{i}"))]
    Int(i64),
    #[delve(display = |i| format!("{i}"))]
    Float(f64),
    #[delve(display = |i| format!("{i}"))]
    Bool(bool),
    #[delve(display = || "()")]
    Empty,
}

impl Eq for Constant {}

impl std::hash::Hash for Constant {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Constant::Int(v) => v.hash(state),
            Constant::Float(v) => v.to_bits().hash(state),
            Constant::Bool(v) => v.hash(state),
            Constant::Empty => "()".hash(state), // idfk lol
        }
    }
}
