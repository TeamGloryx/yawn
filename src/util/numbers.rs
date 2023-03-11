use cast::{Error, From};
use super::magic::GInto;
use std::iter::StepBy;
use std::ops::Range;

pub trait IResult<T, E> {}
impl<T, E> IResult<T, E> for Result<T, E> {}

pub trait Coerce: Sized {
    fn coerce(self, range: Range<Self>) -> Self;
}

pub trait Number: Sized + Coerce {
    const ZERO: Self;
    const MIN: Self;
    const MAX: Self;
    const RANGE: Range<Self> = Self::MIN..Self::MAX;

    fn from_number_raw<N: Number>(n: N) -> <Self as From<N>>::Output
    where
        Self: From<N>;
}

macro number_impl($($number:ident$(,)?)+) {
    $(
    impl Number for $number {
        const ZERO: $number = 0 as $number;
        const MIN: $number = $number::MIN;
        const MAX: $number = $number::MAX;

        fn from_number_raw<N: Number>(n: N) -> <$number as From<N>>::Output where $number: From<N> {
            cast::$number(n)
        }
    }

    impl Coerce for $number {
        fn coerce(self, range: Range<$number>) -> Self {
            self.clamp(range.start, range.end)
        }
    }


    impl GInto<usize> for $number {
        fn g_into(self) -> usize {
            self.clamp(usize::MIN as $number, usize::MAX as $number) as usize
        }
    }

    impl<T: Sized, I: Iterator<Item=T>> Step<$number, T> for I {
        fn step(self, step: $number) -> StepBy<I> {
            self.step_by(step.g_into())
        }
    }
    )+
}

number_impl! {
    usize, isize,
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    f32, f64
}

pub fn convert_range<A, B>(range: Range<A>) -> Range<B>
where
    A: From<A, Output = B>,
{
    (A::cast(range.start))..(A::cast(range.end))
}

pub trait Step<S: Sized + Number, T: Sized>: Sized {
    fn step(self, step: S) -> StepBy<Self>;
}