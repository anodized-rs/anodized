use syn::{
    Attribute, Expr, ExprCall, Stmt, parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::{FnSpec, LoopSpec};

pub(crate) fn haxify_fn(spec: &FnSpec, attrs: &mut Vec<Attribute>) {
    for precond in &spec.requires {
        let body = haxify_expr(&precond.expr);
        attrs.push(parse_quote! { #[::hax_lib::requires(#body)] });
    }
    for invariant in &spec.maintains {
        let body = haxify_expr(&invariant.expr);
        attrs.push(parse_quote! { #[::hax_lib::requires(#body)] });
        attrs.push(parse_quote! { #[::hax_lib::ensures(|_| #body)] });
    }
    for postcond in &spec.ensures {
        let body = haxify_expr(&postcond.expr);
        if let Some(pat) = &postcond.pat {
            let pat = pat.get_pat();
            attrs.push(parse_quote! { #[::hax_lib::ensures(|#pat| #body)] });
        } else {
            attrs.push(parse_quote! { #[::hax_lib::ensures(|_| #body)] });
        }
    }
}

pub(crate) fn haxify_impl_or_trait(attrs: &mut Vec<Attribute>) {
    attrs.push(parse_quote! { #[::hax_lib::attributes] });
}

pub(crate) fn haxify_for_loop(spec: &LoopSpec, stmts: &mut Vec<Stmt>) {
    for condition in &spec.maintains {
        if let Expr::Closure(closure) = &condition.expr {
            let pat = &closure.inputs;
            let invariant = haxify_expr(&closure.body);
            stmts.insert(0, parse_quote! { ::hax_lib::loop_invariant!(|#pat| #invariant); });
        } else {
            let invariant = haxify_expr(&condition.expr);
            stmts.insert(0, parse_quote! { ::hax_lib::loop_invariant!(#invariant); });
        }
    }
}

pub(crate) fn haxify_while_loop(spec: &LoopSpec, stmts: &mut Vec<Stmt>) {
    for condition in &spec.maintains {
        let invariant = haxify_expr(&condition.expr);
        stmts.insert(0, parse_quote! { ::hax_lib::loop_invariant!(#invariant); });
    }
    if let Some(expr) = &spec.decreases {
        let variant = haxify_expr(&expr.expr);
        stmts.insert(0, parse_quote! { ::hax_lib::loop_decreases!(#variant); });
    }
}

pub(crate) fn haxify_expr(expr: &Expr) -> Expr {
    let mut expr = expr.clone();
    HaxExprVisitor.visit_expr_mut(&mut expr);
    expr
}

struct HaxExprVisitor;

impl VisitMut for HaxExprVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Macro(invocation) = expr
            && invocation.mac.path.is_ident("opaque")
        {
            *expr = Expr::Verbatim(invocation.mac.tokens.clone());
        }

        visit_mut::visit_expr_mut(self, expr);
    }

    fn visit_expr_call_mut(&mut self, call: &mut ExprCall) {
        if let Expr::Path(path) = call.func.as_mut() {
            if path.path.is_ident("forall") {
                *call.func.as_mut() = parse_quote!(::hax_lib::forall);
            } else if path.path.is_ident("exists") {
                *call.func.as_mut() = parse_quote!(::hax_lib::exists);
            }
        };

        visit_mut::visit_expr_call_mut(self, call);
    }
}
