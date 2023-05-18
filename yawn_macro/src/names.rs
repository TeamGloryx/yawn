use std::default::default;
use proc_macro2::Span;
use syn::*;
use syn::punctuated::{Pair, Punctuated};

pub(crate) const COMPOSABLE_FN_PREFIX: &str = "__COMPOSABLE_";

pub(crate) const PARAM_COMPOSER: &str = "__composer";

pub(crate) const TYPE_COMPOSER: [&str; 4] = ["yawn", "runtime", "composer", "Composer"];

pub(crate) const TYPE_u64: [&str; 1] = ["u64"];

pub(crate) fn to_ty<const N: usize>(path: [&str; N]) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: make_path(path)
    })
}

pub fn make_path<const N: usize>(path: [&str; N]) -> Path {
    Path {
        segments: make_punctuated(path),
        leading_colon: Some(default()),
    }
}

pub fn make_punctuated<T: From<Ident>, P: Default, const N: usize>(
    values: [&str; N],
) -> Punctuated<T, P> {
    let vec = values.to_vec();
    let size = vec.len();

    if size == 0 {
        return Punctuated::new();
    }

    Punctuated::from_iter(vec.into_iter().enumerate().map(|(i, s)| {
        let ident = Ident::new(s, Span::call_site());
        if s.ends_with(";") || (i.saturating_sub(1)) == size {
            Pair::End(T::from(ident))
        } else {
            Pair::Punctuated(T::from(ident), default())
        }
    }))
}
