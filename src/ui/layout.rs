use skia_safe::{IPoint, ISize, Point, Size as SkSize};

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IntOffset {
    pub x: i32,
    pub y: i32,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct IntSize {
    pub width: i32,
    pub height: i32,
}

impl Into<Point> for Offset {
    fn into(self) -> Point {
        let Offset { x, y } = self;
        Point { x, y }
    }
}
impl Into<Offset> for IntOffset {
    fn into(self) -> Offset {
        let IntOffset { x, y } = self;
        let (x, y) = (x as f32, y as f32);
        Offset { x, y }
    }
}
impl Into<Point> for IntOffset {
    fn into(self) -> Point {
        Into::<Offset>::into(self).into()
    }
}
impl Into<IPoint> for IntOffset {
    fn into(self) -> IPoint {
        let IntOffset { x, y } = self;
        IPoint { x, y }
    }
}

impl Into<ISize> for IntSize {
    fn into(self) -> ISize {
        let IntSize { width, height } = self;
        ISize { width, height }
    }
}

impl Into<SkSize> for Size {
    fn into(self) -> SkSize {
        let Size { width, height } = self;
        SkSize { width, height }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LayoutDirection {
    Ltr,
    Rtl,
}

pub mod alignment {
    use super::{IntOffset, IntSize, LayoutDirection};

    // 2D alignments
    pub static TOP_START: BiasAlignment = BiasAlignment(-1f32, -1f32);
    pub static TOP_CENTER: BiasAlignment = BiasAlignment(0f32, -1f32);
    pub static TOP_END: BiasAlignment = BiasAlignment(1f32, -1f32);
    pub static CENTER_START: BiasAlignment = BiasAlignment(-1f32, 0f32);
    pub static CENTER: BiasAlignment = BiasAlignment(0f32, 0f32);
    pub static CENTER_END: BiasAlignment = BiasAlignment(1f32, 0f32);
    pub static BOTTOM_START: BiasAlignment = BiasAlignment(-1f32, 1f32);
    pub static BOTTOM_CENTER: BiasAlignment = BiasAlignment(0f32, 1f32);
    pub static BOTTOM_END: BiasAlignment = BiasAlignment(1f32, 1f32);

    // 1D vertical alignments
    pub static TOP: BiasAlignment = BiasAlignment::vertical(-1f32);
    pub static CENTER_VERTICALLY: BiasAlignment = BiasAlignment::vertical(0f32);
    pub static BOTTOM: BiasAlignment = BiasAlignment::vertical(1f32);

    // 1D horizontal alignments
    pub static START: BiasAlignment = BiasAlignment::horizontal(-1f32);
    pub static CENTER_HORIZONTALLY: BiasAlignment = BiasAlignment::horizontal(0f32);
    pub static END: BiasAlignment = BiasAlignment::horizontal(1f32);

    pub trait Alignment {
        fn align(&self, size: IntSize, space: IntSize, dir: LayoutDirection) -> IntOffset;
    }
    pub trait HorizontalAlignment {
        fn align(&self, size: i32, space: i32, dir: LayoutDirection) -> i32;
    }
    pub trait VerticalAlignment {
        fn align(&self, size: i32, space: i32) -> i32;
    }

    pub struct BiasAlignment(f32, f32);
    impl BiasAlignment {
        const fn horizontal(bias: f32) -> Self {
            Self(bias, f32::NAN)
        }
        const fn vertical(bias: f32) -> Self {
            Self(f32::NAN, bias)
        }
    }
    impl Alignment for BiasAlignment {
        fn align(&self, size: IntSize, space: IntSize, dir: LayoutDirection) -> IntOffset {
            let cr_x = (space.width - size.width) as f32 / 2f32;
            let cr_y = (space.height - size.height) as f32 / 2f32;
            let resolved_h_bias = if dir == LayoutDirection::Ltr {
                self.0
            } else {
                self.0 * -1f32
            };
            let x = cr_x * (1f32 + resolved_h_bias);
            let y = cr_y * (1f32 + self.1);
            IntOffset {
                x: x.round() as i32,
                y: y.round() as i32,
            }
        }
    }
    impl HorizontalAlignment for BiasAlignment {
        fn align(&self, size: i32, space: i32, dir: LayoutDirection) -> i32 {
            let cr = (space - size) as f32 / 2f32;
            let resolved_bias = if dir == LayoutDirection::Ltr {
                self.0
            } else {
                self.0 * -1f32
            };
            (cr * (1f32 + resolved_bias)).round() as i32
        }
    }
    impl VerticalAlignment for BiasAlignment {
        fn align(&self, size: i32, space: i32) -> i32 {
            let cr = (space - size) as f32 / 2f32;
            (cr * (1f32 + self.1)).round() as i32
        }
    }
}
