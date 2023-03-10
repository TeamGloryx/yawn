use tokio::runtime::{Handle, Runtime};
use crate::runtime::ComposableFn;
use crate::runtime::composition::Composer;
use super::recomposer::Recomposer;

pub struct Composition {
    parent: Recomposer,
    effect_job: &'static Handle,
    content: Box<ComposableFn>,
    has_invalidations: bool
}

impl Composition {
    pub fn new(parent: Recomposer, effect: Runtime) -> Composition {
        Composition {
            parent,
            effect_job: effect.handle(),
            content: Box::new(|| {}),
            has_invalidations: false
        }
    }
}

impl Composition {
    pub fn has_invalidations(&self) -> bool {
        self.has_invalidations
    }

    pub fn set_content<F : Fn(Composer, u64)>(&mut self, content: F) {
        self.content = box content;
    }

    pub fn dispose(self) {

    }
}
