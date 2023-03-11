pub trait GInto<T> {
    fn g_into(self) -> T;
}

pub type GOption<T> = impl GInto<Option<T>>;

pub auto trait Thing {}

impl !Thing for () {}

impl<T> GInto<Option<T>> for () {
    fn g_into(self) -> Option<T> {
        None
    }
}

impl<T : Sized + Thing> GInto<T> for T {
    fn g_into(self) -> Option<T> {
        self
    }
}

impl<T : Sized + Thing> GInto<Option<T>> for T {
    fn g_into(self) -> Option<T> {
        Some(self)
    }
}