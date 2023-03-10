#![feature(box_syntax)]
#![feature(default_free_fn)]
#![feature(is_some_and)]
extern crate proc_macro;

use proc_macro::TokenStream;
mod composable;
pub(crate) mod names;

#[proc_macro_attribute]
pub fn composable(_args: TokenStream, input: TokenStream) -> TokenStream {
    composable::composable_impl(input).into()
}
