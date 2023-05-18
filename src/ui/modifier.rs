use skia_safe::Paint;

use super::layout::alignment::Alignment;

pub(crate) enum BuiltinModifier {
    Background(Paint),
    Align(&'static dyn Alignment),
}

enum ModifierInner {
    Combine(Box<(ModifierInner, ModifierInner)>),
    Modifier(BuiltinModifier),
}

pub struct Modifier(ModifierInner);

impl Modifier {
    pub(crate) fn new(builtin: BuiltinModifier) -> Self {
        Self(ModifierInner::Modifier(builtin))
    }

    pub fn then(self, other: Modifier) -> Self {
        self._then(other.0)
    }
    fn _then(self, other: ModifierInner) -> Self {
        Self(ModifierInner::Combine(Box::new((self.0, other))))
    }
    pub(crate) fn then_(self, other: BuiltinModifier) -> Self {
        self._then(ModifierInner::Modifier(other))
    }

    pub fn into_iter(self) -> impl Iterator<Item = Modifier> {
        match self.0 {
            ModifierInner::Combine(box (a, b)) => a.into_iter().chain(b.into_iter()),
            _ => [self].into_iter(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Modifier> {
        match &self.0 {
            ModifierInner::Combine(box (a, b)) => a.iter().chain(b.iter()),
            _ => [self].iter(),
        }
    }
}
