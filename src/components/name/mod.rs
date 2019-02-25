use specs::{Component, VecStorage};

pub struct Name {
    pub s: String,
}

impl Component for Name {
    type Storage = VecStorage<Self>;
}
