use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Attribute, Block, FnArg, Ident, ItemConst, ItemFn, ItemImpl, ItemTrait, Result, ReturnType,
    Signature, parse_quote,
};

use crate::{DataSpec, Spec};

pub mod data;
pub mod fns;
pub mod loops;
pub mod traits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Make no changes to the code.
    ChangeNothing,
    /// Inject code to enable compile-time and/or runtime checks.
    InjectChecks(CheckSettings),
    /// Embed spec elements as new items without changing existing code.
    EmbedSpecs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckSettings {
    pub does_print: bool,
    pub does_panic: bool,
    pub split_func: bool,
}

impl Mode {
    pub fn changes_anything(&self) -> bool {
        !matches!(self, Mode::ChangeNothing)
    }

    pub fn instrument_item_fn(&self, spec: Spec, mut item_fn: ItemFn) -> Result<TokenStream> {
        let mut tokens = TokenStream::new();

        if let Self::EmbedSpecs = self {
            // Embed `spec` elements as `__anodized_fn_*` items.
            let attrs: [Attribute; 2] = [
                parse_quote!(#[doc(hidden)]),
                parse_quote!(#[allow(warnings)]),
            ];

            let spec_qualifiers_const: ItemConst = Self::build_qualifier_const_item(
                &attrs,
                "__anodized_fn_qualifiers",
                spec.qualifiers,
                &item_fn.sig.ident,
            );
            let spec_requires_fn = ItemFn {
                attrs: attrs.to_vec(),
                vis: syn::Visibility::Inherited,
                sig: Self::build_precondition_fn_sig("__anodized_fn_requires", &item_fn.sig),
                block: Box::new(Self::build_precondition_fn_body(
                    &spec.requires,
                    &spec.maintains,
                )),
            };
            let spec_ensures_fn = ItemFn {
                attrs: attrs.to_vec(),
                vis: syn::Visibility::Inherited,
                sig: Self::build_postcondition_fn_sig("__anodized_fn_ensures", &item_fn.sig),
                block: Box::new(Self::build_postcondition_fn_body(
                    &spec.maintains,
                    &spec.captures,
                    &spec.ensures,
                    &item_fn.sig.output,
                )?),
            };

            spec_qualifiers_const.to_tokens(&mut tokens);
            spec_requires_fn.to_tokens(&mut tokens);
            spec_ensures_fn.to_tokens(&mut tokens);
        }

        // Instrument function body.
        self.instrument_fn(&spec, &item_fn.sig, &mut item_fn.block)?;

        if let Self::InjectChecks(settings) = self
            && settings.split_func
        {
            // Build a wrapper that forwards to the "split" function.
            let mut wrapper_fn = item_fn.clone();
            let mangled_ident =
                Self::build_split_fn(false, &mut wrapper_fn.sig, wrapper_fn.block.as_mut());
            wrapper_fn.to_tokens(&mut tokens);

            // "Split" the original function by mangling its return type.
            // The "split" entry point is used for e.g. fuzzing and PBT.
            item_fn.sig.ident = mangled_ident;
            item_fn.sig.output = match item_fn.sig.output {
                ReturnType::Default => parse_quote!(-> Result<(), (bool, String)>),
                ReturnType::Type(ra, ty) => parse_quote!(#ra Result<#ty, (bool, String)>),
            };
            item_fn.attrs = vec![parse_quote!(#[doc(hidden)]), parse_quote!(#[inline])];
        }

        item_fn.to_tokens(&mut tokens);
        Ok(tokens)
    }

    fn build_split_fn(is_impl: bool, sig: &mut Signature, body: &mut Block) -> Ident {
        let mangled_ident =
            Ident::new(&format!("__anodized_split_{}", sig.ident), sig.ident.span());

        Self::build_wrapper_fn_signature(sig);

        let args = sig.inputs.iter().map(|arg| match arg {
            FnArg::Receiver(receiver) => receiver.self_token.to_token_stream(),
            FnArg::Typed(pat_type) => pat_type.pat.to_token_stream(),
        });

        let maybe_self = match is_impl {
            true => quote::quote!(Self::),
            false => quote::quote!(),
        };

        let maybe_await = match &sig.asyncness {
            Some(_) => quote::quote!(.await),
            None => quote::quote!(),
        };

        *body = parse_quote! {
            {
                match #maybe_self #mangled_ident(#(#args),*) #maybe_await {
                    Ok(output) => output,
                    Err((false, errors)) => panic!("precondition failed:{errors}"),
                    Err((true, errors)) => panic!("postcondition failed:{errors}"),
                }
            }
        };

        mangled_ident
    }

    fn build_wrapper_fn_signature(sig: &mut Signature) {
        use syn::spanned::Spanned;
        for (i, arg) in sig.inputs.iter_mut().enumerate() {
            match arg {
                FnArg::Receiver(_) => {}
                FnArg::Typed(pat_type) => {
                    let name = Ident::new(&format!("input_{i}"), pat_type.span());
                    pat_type.pat = parse_quote!(#name);
                }
            }
        }
    }

    pub fn instrument_item_trait(
        &self,
        spec: DataSpec,
        item_trait: ItemTrait,
    ) -> Result<TokenStream> {
        let new_trait = self.instrument_trait(spec, item_trait)?;
        Ok(new_trait.to_token_stream())
    }

    pub fn instrument_item_trait_impl(
        &self,
        spec: DataSpec,
        item_impl: ItemImpl,
    ) -> Result<TokenStream> {
        let new_trait_impl = self.instrument_trait_impl(spec, item_impl)?;
        Ok(new_trait_impl.to_token_stream())
    }
}

#[cfg(test)]
impl Mode {
    pub(crate) const DEFAULT: Self = Mode::InjectChecks(CheckSettings::DEFAULT);
}

#[cfg(test)]
impl CheckSettings {
    pub(crate) const DEFAULT: Self = Self {
        does_print: false,
        does_panic: false,
        split_func: false,
    };

    pub(crate) const PRINT: Self = Self {
        does_print: true,
        does_panic: false,
        split_func: false,
    };

    pub(crate) const PRINT_AND_PANIC: Self = Self {
        does_print: true,
        does_panic: true,
        split_func: false,
    };

    pub(crate) const PRINT_AND_SPLIT_PANIC: Self = Self {
        does_print: true,
        does_panic: true,
        split_func: true,
    };
}

/// Make an error message to say that some item is unsupported.
pub fn make_item_error<T: ToTokens>(tokens: &T, item_descr: &str) -> syn::Error {
    let msg = format!(
        r#"The #[spec] attribute doesn't yet support this item: {}.
If this is a problem for your use case, please open a feature
request at https://github.com/anodized-rs/anodized/issues/new"#,
        item_descr
    );
    syn::Error::new_spanned(tokens, msg)
}

/// Finds the `[spec]` attrib in an attribute list.
///
/// Returns the spec [Attribute] and the remaining attributes.
fn find_spec_attr(attrs: Vec<Attribute>) -> syn::Result<(Option<Attribute>, Vec<Attribute>)> {
    let mut spec_attr = None;
    let mut other_attrs = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("spec") {
            if spec_attr.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "multiple `#[spec]` attributes on a single item are not supported",
                ));
            }
            spec_attr = Some(attr);
        } else {
            other_attrs.push(attr);
        }
    }

    Ok((spec_attr, other_attrs))
}
