use std::ops::{Deref, DerefMut};

use crate::primitive_types::Vector2I;

#[derive(Debug, Default, Copy, Clone)]
pub struct Size(pub Vector2I);

impl Deref for Size {
    type Target = Vector2I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Size {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
