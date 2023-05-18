use crate::ui::render::RenderContext;
use skia_safe::{Color, ColorSpace, Color3f};
use skia_safe::{Canvas, Color4f, Paint};
use crate::ui::layout::alignment::Alignment;
use super::modifier::Modifier;
use super::modifier::BuiltinModifier;

trait ToPaint {
    fn to_paint(&self) -> Paint;
}

impl ToPaint for Color {
    fn to_paint(&self) -> Paint {
        Paint::new(self, ColorSpace::new_srgb())
    }
}

impl ToPaint for Color3f {
    fn to_paint(&self) -> Paint {
        Paint::new(Color4f::new(self.r, self.g, self.b, 1f32), ColorSpace::new_srgb())
    }
}

impl Modifier {
    pub fn background(self, paint: &impl ToPaint) -> Self {
        self.then_(BuiltinModifier::Background(paint.to_paint()))
    }
    
    pub fn align(self, alignment: &'static dyn Alignment) -> Self {
        self.then_(BuiltinModifier::Align(alignment))
    }
}
