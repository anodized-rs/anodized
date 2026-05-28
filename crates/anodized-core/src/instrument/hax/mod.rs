use syn::{
    Attribute, Expr, ExprCall, Pat, Stmt, parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::{LoopSpec, Spec};

pub(crate) fn haxify_fn(spec: &Spec, attrs: &mut Vec<Attribute>) {
    for precond in &spec.requires {
        let body = haxify_expr(&precond.closure.body);
        attrs.push(parse_quote! { #[::hax_lib::requires(#body)] });
    }
    for invariant in &spec.maintains {
        let body = haxify_expr(&invariant.closure.body);
        attrs.push(parse_quote! { #[::hax_lib::requires(#body)] });
        attrs.push(parse_quote! { #[::hax_lib::ensures_ref(|_| #body)] });
    }
    for postcond in &spec.ensures {
        let inputs = &postcond.closure.inputs;
        let body = haxify_expr(&postcond.closure.body);
        attrs.push(parse_quote! { #[::hax_lib::ensures_ref(|#inputs| #body)] });
    }
}

pub(crate) fn haxify_impl_or_trait(attrs: &mut Vec<Attribute>) {
    attrs.push(parse_quote! { #[::hax_lib::attributes] });
}

pub(crate) fn haxify_for_loop(spec: &LoopSpec, pat: &Pat, stmts: &mut Vec<Stmt>) {
    for condition in &spec.maintains {
        let invariant = haxify_expr(&condition.closure.body);
        stmts.push(parse_quote! { ::hax_lib::loop_invariant!(|#pat| #invariant); });
    }
}

pub(crate) fn haxify_while_loop(spec: &LoopSpec, stmts: &mut Vec<Stmt>) {
    for condition in &spec.maintains {
        let invariant = haxify_expr(&condition.closure.body);
        stmts.push(parse_quote! { ::hax_lib::loop_invariant!(#invariant); });
    }
    if let Some(expr) = &spec.decreases {
        let variant = haxify_expr(&expr.expr);
        stmts.push(parse_quote! { ::hax_lib::loop_decreases!(#variant); });
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
        if let Expr::Macro(invocation) = expr {
            if invocation.mac.path.is_ident("opaque") {
                *expr = Expr::Verbatim(invocation.mac.tokens.clone());
            }
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
