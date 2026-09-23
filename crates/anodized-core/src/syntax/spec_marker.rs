use quote::ToTokens;
use syn::{
    Type, TypeMacro,
    parse::{Parse, ParseStream, Result},
    token::Comma,
};

use crate::syntax::path_matches_name;

/// The `spec!(TYPE[, MODE])` marker for fine-grained control of type spec enforcement.
#[derive(Debug)]
pub struct SpecMarker {
    pub ty: Type,
    /// Must be `None` on the output of a `fn` or field of a data type (`struct` or `enum`).
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

impl Parse for SpecMarker {
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
pub fn extract_spec_marker(ty: &mut Type) -> Result<Option<SpecMarker>> {
    let Type::Macro(TypeMacro { mac }) = ty else {
        return Ok(None);
    };
    if !path_matches_name(&mac.path, "spec") {
        return Ok(None);
    }

    let spec_marker: SpecMarker = syn::parse2(mac.tokens.clone())?;
    *ty = spec_marker.ty.clone();
    Ok(Some(spec_marker))
}

mod kw {
    syn::custom_keyword!(out);
    syn::custom_keyword!(inout);
}
