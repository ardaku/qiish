#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::suspicious)]

use std::array::IntoIter;

/// The Cursor-based type used internally to implement a type that can look ahead and behind without
/// consuming the item.
#[derive(Debug, Clone)]
pub struct Lookahead<T: Sized> {
    inner: Vec<T>,
    peek_cursor: usize,
    real_cursor: usize,
}

impl<T: Sized + Clone> Lookahead<T> {
    /// Creates a new Lookahead from a [`Vec`].
    #[must_use]
    pub const fn new(inner: Vec<T>) -> Self {
        Self {
            inner,
            peek_cursor: 0usize,
            real_cursor: 0usize,
        }
    }

    /// Returns the current item.
    #[must_use]
    pub fn current(&mut self) -> Option<T> {
        self.inner.get(self.peek_cursor).cloned()
    }

    /// Moves the cursor forward by `n` items.
    pub fn forward(&mut self, n: usize) {
        self.peek_cursor += n;
    }

    /// Moves the cursor backward by `n` items.
    pub fn backward(&mut self, n: usize) {
        self.peek_cursor -= n;
    }

    /// Moves the peek cursor to the real cursor.
    pub fn reset_cursor(&mut self) {
        self.peek_cursor = self.real_cursor;
    }
}

impl<T: Sized + Clone> Iterator for Lookahead<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.get(self.peek_cursor).cloned();
        self.real_cursor += 1;
        self.peek_cursor += 1;
        ret
    }
}

impl<T: Sized + Clone> From<Vec<T>> for Lookahead<T> {
    fn from(self_: Vec<T>) -> Self {
        Self::new(self_)
    }
}

impl<T: Sized + Clone> From<Lookahead<T>> for Vec<T> {
    fn from(self_: Lookahead<T>) -> Self {
        self_.inner
    }
}

impl<T: Sized + Clone> FromIterator<T> for Lookahead<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}
