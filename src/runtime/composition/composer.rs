use crate::runtime::{Applier, Node};

pub struct Composer {
    applier: Box<dyn Applier<dyn Node>>
}

impl Composer {
    fn changed_bool(&self, value: bool) -> bool {
        false
    }
}