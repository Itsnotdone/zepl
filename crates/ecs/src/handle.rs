use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Handle<T> {
    pub index: usize,
    _phantom: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            _phantom: PhantomData,
        }
    }
}
