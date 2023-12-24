use super::value::StoredValue;
use crate::util::slabmap::SlabMap;

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Into, derive_more::From,
)]
pub struct MemKey(pub usize);

pub type Memory = SlabMap<MemKey, StoredValue>;
