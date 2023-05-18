pub trait YInto<T> {
    fn y_into(self) -> T;
}

pub macro option($t:ty) {
    impl YInto<Option<$t>>
}

pub auto trait Thing {}

impl !Thing for () {}

impl<T: Sized + Thing> YInto<Option<T>> for () {
    fn y_into(self) -> Option<T> {
        None
    }
}

impl<T: Sized + Thing> YInto<Option<T>> for T {
    fn y_into(self) -> Option<T> {
        Some(self)
    }
}
