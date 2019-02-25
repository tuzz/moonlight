use specs::prelude::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Name {
    pub string: String,
}

impl Name {
    pub fn new(s: &str) -> Self {
        Self { string: s.to_string() }
    }
}
