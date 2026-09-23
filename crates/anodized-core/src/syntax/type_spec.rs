use quote::ToTokens;
use syn::{
    Type, TypeMacro,
    parse::{Parse, ParseStream, Result},
    token::Comma,
};

use crate::syntax::path_matches_name;

/// A `spec!(TYPE[, MODE])` enforcement type marker.
#[derive(Debug)]
pub struct SpecTypeMarker {
    pub ty: Type,
    /// Must be `None` for the output of a `fn` or field of a `struct` or `enum`.
    pub mode: Option<(Comma, FnArgMode)>,
}

/// The enforcement mode of a `fn` input.
#[derive(Debug)]
pub enum FnArgMode {
    Out(kw::out),
    InOut(kw::inout),
}

impl ToTokens for FnArgMode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FnArgMode::Out(out) => out.to_tokens(tokens),
            FnArgMode::InOut(inout) => inout.to_tokens(tokens),
        }
    }
}

impl Parse for SpecTypeMarker {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse()?;
        let mode = if input.is_empty() {
            None
        } else {
            let comma = input.parse::<Comma>()?;
            if input.peek(kw::out) {
                Some((comma, FnArgMode::Out(input.parse()?)))
            } else if input.peek(kw::inout) {
                Some((comma, FnArgMode::InOut(input.parse()?)))
            } else {
                return Err(input.error("expected a mode, `out` or `inout`"));
            }
        };

        if !input.is_empty() {
            return Err(input.error("expected exactly one mode, `out` or `inout`"));
        }

        Ok(Self { ty, mode })
    }
}

/// Removes a `spec!(...)` marker from a type, if it has one.
pub fn extract_type_spec(ty: &mut Type) -> Result<Option<SpecTypeMarker>> {
    let Type::Macro(TypeMacro { mac }) = ty else {
        return Ok(None);
    };
    if !path_matches_name(&mac.path, "spec") {
        return Ok(None);
    }

    let type_spec: SpecTypeMarker = syn::parse2(mac.tokens.clone())?;
    *ty = type_spec.ty.clone();
    Ok(Some(type_spec))
}

mod kw {
    syn::custom_keyword!(out);
    syn::custom_keyword!(inout);
}
