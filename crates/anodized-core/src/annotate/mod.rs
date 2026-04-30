use syn::{
    Attribute, Expr, Ident, Meta, Pat, PatIdent,
    parse::{Parse, ParseStream, Result},
    parse_quote,
    spanned::Spanned,
};

use crate::{
    Capture, PostCondition, PreCondition, Spec,
    annotate::syntax::{CaptureExpr, SpecArgValue},
};

pub mod syntax;
use syntax::{Captures, Keyword};

#[cfg(test)]
mod tests;

impl Parse for Spec {
    fn parse(input: ParseStream) -> Result<Self> {
        let raw_spec = syntax::SpecArgs::parse(input)?;

        let mut prev_keyword: Option<Keyword> = None;
        let mut requires: Vec<PreCondition> = vec![];
        let mut maintains: Vec<PreCondition> = vec![];
        let mut captures: Vec<Capture> = vec![];
        let mut binds_pattern: Option<Pat> = None;
        let mut ensures: Vec<PostCondition> = vec![];

        for arg in raw_spec.args {
            match &arg.keyword {
                Keyword::Requires => arg.value.parse_preconditions(&arg.attrs, &mut requires)?,
                Keyword::Maintains => arg.value.parse_preconditions(&arg.attrs, &mut maintains)?,
                Keyword::Captures => {
                    if !captures.is_empty() {
                        return Err(syn::Error::new(
                            arg.keyword_span,
                            "at most one `captures` parameter is allowed; to capture multiple values, use a list: `captures: [expr1, expr2, ...]`",
                        ));
                    }
                    arg.value.parse_captures(&arg.attrs, &mut captures)?
                }
                Keyword::Binds => {
                    if binds_pattern.is_some() {
                        return Err(syn::Error::new(
                            arg.keyword_span,
                            "multiple `binds` parameters are not allowed",
                        ));
                    }
                    arg.value.parse_binds(&arg.attrs, &mut binds_pattern)?
                }
                Keyword::Ensures => {
                    arg.value
                        .parse_postconditions(&arg.attrs, &binds_pattern, &mut ensures)?
                }
                Keyword::Unknown(ident) => {
                    return Err(syn::Error::new(
                        arg.keyword_span,
                        format!("unknown spec keyword `{ident}`"),
                    ));
                }
            }

            if let Some(prev_keyword) = prev_keyword
                && arg.keyword < prev_keyword
            {
                return Err(syn::Error::new(
                    arg.keyword_span,
                    "parameters are out of order: their order must be `requires`, `maintains`, `captures`, `binds`, `ensures`",
                ));
            }
            prev_keyword = Some(arg.keyword);
        }

        Ok(Spec {
            requires,
            maintains,
            captures,
            ensures,
            span: input.span(),
        })
    }
}

impl SpecArgValue {
    fn parse_preconditions(
        self,
        attrs: &[Attribute],
        preconditions: &mut Vec<PreCondition>,
    ) -> Result<()> {
        let cfg_attr = find_cfg_attribute(attrs)?;
        let cfg: Option<Meta> = if let Some(attr) = cfg_attr {
            Some(attr.parse_args()?)
        } else {
            None
        };
        let expr = self.try_into_expr()?;
        if let Expr::Array(conditions) = expr {
            for expr in conditions.elems {
                preconditions.push(PreCondition {
                    closure: interpret_expr_as_precondition(expr)?,
                    cfg: cfg.clone(),
                });
            }
        } else {
            preconditions.push(PreCondition {
                closure: interpret_expr_as_precondition(expr)?,
                cfg,
            });
        }
        Ok(())
    }

    fn parse_captures(self, attrs: &[Attribute], captures: &mut Vec<Capture>) -> Result<()> {
        let cfg_attr = find_cfg_attribute(attrs)?;
        if cfg_attr.is_some() {
            return Err(syn::Error::new(
                cfg_attr.span(),
                "`cfg` attribute is not supported on `captures`",
            ));
        }
        let capture_list = self.try_into_captures()?;
        match capture_list {
            Captures::One(capture_expr) => {
                captures.push(interpret_capture_expr_as_capture(*capture_expr.clone())?);
            }
            Captures::Many { elems, .. } => {
                for capture_expr in elems {
                    captures.push(interpret_capture_expr_as_capture(capture_expr)?);
                }
            }
        }
        Ok(())
    }

    fn parse_binds(self, attrs: &[Attribute], pattern: &mut Option<Pat>) -> Result<()> {
        let cfg_attr = find_cfg_attribute(attrs)?;
        if cfg_attr.is_some() {
            return Err(syn::Error::new(
                cfg_attr.span(),
                "`cfg` attribute is not supported on `binds`",
            ));
        }
        let binds_pattern = self.try_into_pat()?;
        *pattern = Some(binds_pattern);
        Ok(())
    }

    fn parse_postconditions(
        self,
        attrs: &[Attribute],
        binds_pattern: &Option<Pat>,
        postconditions: &mut Vec<PostCondition>,
    ) -> Result<()> {
        let cfg_attr = find_cfg_attribute(attrs)?;
        let cfg: Option<Meta> = if let Some(attr) = cfg_attr {
            Some(attr.parse_args()?)
        } else {
            None
        };
        let expr = self.try_into_expr()?;
        let default_pattern = binds_pattern.clone().unwrap_or(parse_quote! { output });
        if let Expr::Array(conditions) = expr {
            for expr in conditions.elems {
                postconditions.push(PostCondition {
                    closure: interpret_expr_as_postcondition(expr, default_pattern.clone())?,
                    cfg: cfg.clone(),
                });
            }
        } else {
            postconditions.push(PostCondition {
                closure: interpret_expr_as_postcondition(expr, default_pattern)?,
                cfg,
            });
        }
        Ok(())
    }
}

/// Try to interpret a CaptureExpr as a single Capture
fn interpret_capture_expr_as_capture(capture_expr: CaptureExpr) -> Result<Capture> {
    let span = capture_expr.span();
    let CaptureExpr { expr, as_, pat } = capture_expr;

    match (expr, as_, pat) {
        // Complete form: <expression> `as` <pattern>
        (Some(expr), Some(_), Some(pat)) => Ok(Capture { expr, pat }),

        // Shorthand: <identifier>
        (Some(ref expr @ Expr::Path(ref path)), None, None)
            if path.path.segments.len() == 1
                && path.path.leading_colon.is_none()
                && path.attrs.is_empty()
                && path.qself.is_none() =>
        {
            // auto-generate binding with `old_` prefix
            let ident = &path.path.segments[0].ident;
            let ident_alias = Ident::new(&format!("old_{}", ident), ident.span());
            let pat = Pat::Ident(PatIdent {
                ident: ident_alias,
                attrs: vec![],
                mutability: None,
                by_ref: None,
                subpat: None,
            });
            Ok(Capture {
                expr: expr.clone(),
                pat,
            })
        }

        // Missing <pattern>
        (Some(_), Some(_), None) => {
            Err(syn::Error::new_spanned(as_, "expected pattern after `as`"))
        }

        // Missing `as` and <pattern>
        (Some(expr), None, None) => Err(syn::Error::new_spanned(
            expr,
            "complex expression must be bound/descructured: <expression> `as` <pattern>",
        )),

        // Missing `as`
        (Some(_), None, Some(pat)) => Err(syn::Error::new_spanned(pat, "expected `as` <pattern>")),

        // Missing <expression>
        (None, _, _) => Err(syn::Error::new(
            span,
            "expected capture: <expression> `as` <pattern>",
        )),
    }
}

/// Interpret expression as a zero-parameter closure, wrapping if necessary.
/// Used for preconditions which don't need access to the return value.
fn interpret_expr_as_precondition(expr: Expr) -> Result<syn::ExprClosure> {
    match expr {
        // Already a closure, validate it has no arguments.
        Expr::Closure(closure) => {
            if closure.inputs.is_empty() {
                Ok(closure)
            } else {
                Err(syn::Error::new_spanned(
                    closure.or1_token,
                    format!(
                        "precondition closure must have no arguments, found {}",
                        closure.inputs.len()
                    ),
                ))
            }
        }
        // Naked expression, wrap in an argumentless closure.
        expr => Ok(syn::ExprClosure {
            attrs: vec![],
            lifetimes: None,
            constness: None,
            movability: None,
            asyncness: None,
            capture: None,
            or1_token: Default::default(),
            inputs: syn::punctuated::Punctuated::new(),
            or2_token: Default::default(),
            output: syn::ReturnType::Default,
            body: Box::new(expr),
        }),
    }
}

/// Interpret expression as a closure with a single argument (eg the list of
/// aliases and function result), wrapping if necessary.
/// Used for postconditions which take the return value as an argument.
fn interpret_expr_as_postcondition(expr: Expr, default_binding: Pat) -> Result<syn::ExprClosure> {
    match expr {
        // Already a closure, validate it has exactly one argument.
        Expr::Closure(closure) => {
            if closure.inputs.len() == 1 {
                Ok(closure)
            } else {
                Err(syn::Error::new_spanned(
                    closure.or1_token,
                    format!(
                        "postcondition closure must have exactly one argument, found {}",
                        closure.inputs.len()
                    ),
                ))
            }
        }
        // Naked expression, wrap in a closure with default binding.
        expr => Ok(syn::ExprClosure {
            attrs: vec![],
            lifetimes: None,
            constness: None,
            movability: None,
            asyncness: None,
            capture: None,
            or1_token: Default::default(),
            inputs: syn::punctuated::Punctuated::from_iter([default_binding]),
            or2_token: Default::default(),
            output: syn::ReturnType::Default,
            body: Box::new(expr),
        }),
    }
}

fn find_cfg_attribute(attrs: &[Attribute]) -> Result<Option<&Attribute>> {
    let mut cfg_attr: Option<&Attribute> = None;

    for attr in attrs {
        if attr.path().is_ident("cfg") {
            if cfg_attr.is_some() {
                return Err(syn::Error::new(
                    attr.span(),
                    "multiple `cfg` attributes are not supported",
                ));
            }
            cfg_attr = Some(attr);
        } else {
            return Err(syn::Error::new(
                attr.span(),
                "unsupported attribute; only `cfg` is allowed",
            ));
        }
    }

    Ok(cfg_attr)
}
