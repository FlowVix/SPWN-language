use crate::source::CodeSpan;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SubstitutionPart {
    pub span: CodeSpan,
    pub snippet: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Substitution {
    pub parts: Vec<SubstitutionPart>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Suggestion {
    pub subsitutions: Vec<Substitution>,
    pub message: String,
}
