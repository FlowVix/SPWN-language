use std::cell::RefCell;
use std::rc::Rc;

use ahash::RandomState;
use lasso::{Rodeo, Spur};

use super::ImmutStr;

#[derive(Debug)]
pub struct Interner(Rc<RefCell<Rodeo<Spur, RandomState>>>);

impl Default for Interner {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(
            Rodeo::with_hasher(RandomState::new()),
        )))
    }
}

impl Interner {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn get_or_intern<T>(&self, val: T) -> Spur
    where
        T: AsRef<str>,
    {
        self.0.borrow_mut().get_or_intern(val)
    }

    // #[inline(always)]
    // pub fn resolve(&self, s: &Spur) -> &str {
    //     self.0.borrow().resolve(s)
    // }

    #[inline(always)]
    pub fn resolve_immut(&self, s: &Spur) -> ImmutStr {
        self.0.borrow().resolve(s).into()
    }
}

impl Clone for Interner {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
