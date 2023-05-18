pub trait Element {
    fn new<W: Widget>(widget: &W) -> Self;
}

pub trait Widget {
    fn _create_element(&self) -> impl Element;
    fn build(&self) -> impl Widget;
}
