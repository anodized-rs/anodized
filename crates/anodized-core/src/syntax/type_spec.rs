use proc_macro2::Span;
use syn::{
    Type, TypeMacro,
    parse::{Parse, ParseStream, Result},
    spanned::Spanned,
};

use crate::syntax::path_matches_name;

/// A `spec!(TYPE[, MODE])` type marker.
#[derive(Debug)]
pub struct TypeSpec {
    pub ty: Type,
    pub mode: Option<TypeSpecMode>,
    pub span: Span,
}

/// The optional enforcement mode of a type marker.
#[derive(Debug)]
pub enum TypeSpecMode {
    Out(kw::out),
    InOut(kw::inout),
}

impl Parse for TypeSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        let mode = if input.is_empty() {
            None
        } else {
            input.parse::<syn::Token![,]>()?;
            if input.peek(kw::out) {
                Some(TypeSpecMode::Out(input.parse()?))
            } else if input.peek(kw::inout) {
                Some(TypeSpecMode::InOut(input.parse()?))
            } else {
                return Err(input.error("expected `out` or `inout`"));
            }
        };

        if !input.is_empty() {
            return Err(input.error("expected exactly one enforcement mode"));
        }

        Ok(Self {
            ty,
            mode,
            span: input.span(),
        })
    }
}

/// Removes a `spec!(...)` marker from a type, if it has one.
pub fn extract_type_spec(ty: &mut Type) -> Result<Option<TypeSpec>> {
    let Type::Macro(TypeMacro { mac, .. }) = ty else {
        return Ok(None);
    };
    if !path_matches_name(&mac.path, "spec") {
        return Ok(None);
    }

    let span = mac.span();
    let mut type_spec: TypeSpec = syn::parse2(mac.tokens.clone())?;
    type_spec.span = span;
    *ty = type_spec.ty.clone();
    Ok(Some(type_spec))
}

mod kw {
    syn::custom_keyword!(out);
    syn::custom_keyword!(inout);
}
