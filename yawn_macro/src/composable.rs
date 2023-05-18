use crate::names::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use std::default::default;
use std::ops::{Deref, Index};
use syn::ext::IdentExt;
use syn::parse::ParseStream;
use syn::punctuated::{Pair, Punctuated};
use syn::spanned::Spanned;
use syn::token::{Colon, Colon2, Mut};
use syn::visit_mut::{visit_item_fn_mut, VisitMut};
use syn::{
    parse, parse_macro_input, Expr, ExprCall, ExprClosure, ExprMethodCall, ExprPath, ExprReference,
    ExprType, FnArg, ItemFn, Lifetime, Pat, PatIdent, PatType, Path, PathArguments, PathSegment,
    Stmt, Token, Type, TypePath, TypeReference,
};

struct ComposableFn;

impl ComposableFn {
    fn is_composable_fn(f: &ExprMethodCall) -> bool {
        f.method
            .unraw()
            .to_string()
            .starts_with(COMPOSABLE_FN_PREFIX)
    }

    fn is_composable_lambda(l: &ExprCall) -> bool {
        l.args.first().is_some_and(|x| {
            if let Expr::Type(ExprType {
                ty,
                expr: _expr,
                colon_token: _colon_token,
                attrs: _attrs,
            }) = x
            {
                match ty.deref() {
                    Type::Path(TypePath {
                        qself: _qself,
                        path,
                    }) => {
                        path.to_token_stream().to_string()
                            == make_path(TYPE_COMPOSER).to_token_stream().to_string()
                    }
                    _ => false,
                }
            } else {
                false
            }
        })
    }

    fn make_arg_in_call<const N: usize>(ty: [&str; N], param: &str) -> Expr {
        Expr::Type(ExprType {
            attrs: default(),
            expr: Box::new(Expr::Reference(ExprReference {
                raw: default(),
                expr: Box::new(Expr::Path(ExprPath {
                    attrs: default(),
                    qself: default(),
                    path: make_path([param]),
                })),
                attrs: default(),
                mutability: default(),
                and_token: default(),
            })),
            ty: Box::new(ty_of(ty)),
            colon_token: default(),
        })
    }

    fn prepend_composable(pu: &mut Punctuated<Expr, Token!(,)>) {
        Self::prepend_params(
            pu,
            vec![
                Self::make_arg_in_call(TYPE_COMPOSER, PARAM_COMPOSER)
            ],
        )
    }

    fn prepend_params<T: Clone, P: Default>(pu: &mut Punctuated<T, P>, args: Vec<T>) {
        let mut n = Punctuated::<T, P>::new();
        n.extend(args);

        *pu = n;
    }
}

impl VisitMut for ComposableFn {
    fn visit_expr_call_mut(&mut self, i: &mut ExprCall) {
        if !Self::is_composable_lambda(i) {
            return;
        }

        Self::prepend_composable(&mut i.args);
    }

    fn visit_expr_method_call_mut(&mut self, i: &mut ExprMethodCall) {
        if !Self::is_composable_fn(i) {
            return;
        }

        Self::prepend_composable(&mut i.args);
    }

    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        Self::prepend_params(
            &mut i.sig.inputs,
            vec![
                FnArg::Typed(cmp_arg_pt(TYPE_COMPOSER, PARAM_COMPOSER)),
            ],
        );

        visit_item_fn_mut(self, i);
    }
}

pub(crate) fn composable_fn_impl(mut func: ItemFn) -> TokenStream {
    ComposableFn.visit_item_fn_mut(&mut func);

    func.to_token_stream()
}

fn ty_of<const N: usize>(ty: [&str; N]) -> Type {
    Type::Reference(TypeReference {
        lifetime: None,
        mutability: None,
        and_token: default(),
        elem: Box::new(Type::Path(TypePath {
            qself: None,
            path: make_path::<N>(ty),
        })),
    })
}

fn cmp_arg_pt<const N: usize>(ty: [&str; N], param: &str) -> PatType {
    PatType {
        pat: Box::new(Pat::Ident(pat_ident(Ident::new(param, Span::call_site())))),
        attrs: Vec::new(),
        ty: Box::new(ty_of(ty)),
        colon_token: default(),
    }
}

fn cmp_arg<const N: usize>(ty: [&str; N], param: &str) -> Pat {
    Pat::Type(cmp_arg_pt(ty, param))
}

fn make_composable_type(ty: Type) -> Type {
    fn replace(path: Path) -> Path {
        if path.segments.is_empty() {
            return path;
        }

        let mut segments = path.segments.into_iter().collect::<Vec<_>>();
        let mut popped = segments.pop().unwrap();
        if let PathSegment {
            ident: _ident,
            arguments: PathArguments::Parenthesized(ref mut p),
        } = &mut popped
        {
            ComposableFn::prepend_params(
                &mut p.inputs,
                vec![ty_of(TYPE_COMPOSER), ty_of(TYPE_u64)],
            );
        };

        segments.push(popped);

        Path {
            leading_colon: path.leading_colon,
            segments: Punctuated::from_iter(segments),
        }
    }

    match ty {
        Type::Path(p)
            if p.path
                .segments
                .last()
                .is_some_and(|seg| seg.ident.unraw().to_string().starts_with("Fn")) =>
        {
            Type::Path(TypePath {
                qself: p.qself,
                path: replace(p.path),
            })
        }

        ty => ty,
    }
}

pub(crate) fn visit_lambda_mut(input: &mut ExprClosure) {
    ComposableFn::prepend_params(
        &mut input.inputs,
        vec![
            cmp_arg(TYPE_COMPOSER, PARAM_COMPOSER),
        ],
    );
}

pub(crate) fn composable_impl(input: proc_macro::TokenStream) -> TokenStream {
    if let Ok(item) = parse::<ItemFn>(input.clone()) {
        return composable_fn_impl(item);
    }
    if let Ok(mut closure) = parse::<ExprClosure>(input.clone()) {
        visit_lambda_mut(&mut closure);
        return closure.to_token_stream();
    }
    if let Ok(Pat::Type(ty)) = parse::<Pat>(input.clone()) {
        return PatType {
            pat: ty.pat,
            ty: Box::new(make_composable_type(*ty.ty.clone())),
            attrs: ty.attrs,
            colon_token: ty.colon_token,
        }
        .to_token_stream();
    }
    if let Ok(ty) = parse::<Type>(input.clone()) {
        return make_composable_type(ty).to_token_stream();
    }

    syn::Error::new(
        TokenStream::from(input).span(),
        "invalid target for #[composable]",
    )
    .to_compile_error()
}

fn pat_ident(ident: Ident) -> PatIdent {
    PatIdent {
        attrs: Vec::new(),
        ident,
        by_ref: None,
        subpat: None,
        mutability: None,
    }
}
