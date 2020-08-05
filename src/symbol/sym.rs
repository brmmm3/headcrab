//! Implementation of a symbol table entry that will automatically
//! demangle rustc names.

use addr2line::demangle_auto;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

/// A symbol table entry.
#[derive(Clone, Debug)]
pub struct Symbol<'data> {
    symbol: object::Symbol<'data>,
}

impl<'data> Symbol<'data> {
    /// Returns the demangled name if this symbol has a name.
    #[inline]
    pub fn demangled_name(&self) -> Option<String> {
        // TODO: Avoid this allocation in every call. (lifetime errors)
        self.name()
            .map(|name| demangle_auto(Cow::Borrowed(name), None).to_string())
    }
}

impl<'data> From<object::Symbol<'data>> for Symbol<'data> {
    fn from(symbol: object::Symbol<'data>) -> Self {
        Symbol { symbol }
    }
}

impl<'data> Deref for Symbol<'data> {
    type Target = object::Symbol<'data>;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

impl DerefMut for Symbol<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.symbol
    }
}
