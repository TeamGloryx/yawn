use std::rc::Rc;

use super::owner::Owner;

pub struct LayoutNode {
    pub(crate) owner: Rc<Box<dyn Owner>>,
    pub(crate) depth: usize,
    pub(crate) parent: Option<Rc<LayoutNode>>,
}

impl LayoutNode {
    fn redepth_child(&mut self, child: &mut LayoutNode) {
        if child.depth <= self.depth {
            child.depth = self.depth + 1;
            child.redepth_children();
        }
    }

    fn redepth_children(&mut self) {
        
    }
}
