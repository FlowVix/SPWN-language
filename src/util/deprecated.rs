use ahash::AHashSet;
use allow_until::AllowUntil;

use crate::source::CodeSpan;

#[derive(Default, Debug, Clone, AllowUntil)]
pub struct DeprecatedFeatures {
    // use of `let` instead of `mut`
    #[allow_until(version = ">=1.0.0")]
    pub let_not_mut: AHashSet<CodeSpan>,
}

impl DeprecatedFeatures {
    // used in the parser to merge after cloning
    pub fn extend(&mut self, other: DeprecatedFeatures) {
        self.let_not_mut.extend(other.let_not_mut);
    }
}
