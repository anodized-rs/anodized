#[cfg(test)]
mod tests;

use syn::{
    Block, Error, ExprClosure, ExprForLoop, ExprWhile, ItemFn, Result, Stmt, parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::{
    LoopSpec,
    instrument::{
        Config, find_spec_attr,
        hax::{haxify_for_loop, haxify_while_loop},
    },
};

impl Config {
    pub fn instrument_loops_in_fn_body(&self, body: &mut Block) -> Result<()> {
        let mut visitor = LoopSpecVisitor::new(self);
        visitor.visit_block_mut(body);
        visitor.finish()
    }

    pub fn instrument_expr_while(&self, spec: &LoopSpec, expr_while: &mut ExprWhile) {
        self.instrument_loop_body(spec, &mut expr_while.body.stmts);

        if self.target_hax {
            haxify_while_loop(spec, &mut expr_while.body.stmts);
        }
    }

    pub fn instrument_expr_for_loop(&self, spec: &LoopSpec, expr_for_loop: &mut ExprForLoop) {
        self.instrument_loop_body(spec, &mut expr_for_loop.body.stmts);

        if self.target_hax {
            haxify_for_loop(spec, &expr_for_loop.pat, &mut expr_for_loop.body.stmts);
        }
    }

    fn instrument_loop_body(&self, spec: &LoopSpec, stmts: &mut Vec<Stmt>) {
        if self.embed_spec {
            let maintains_block = Self::build_precondition_fn_body(&spec.maintains);
            stmts.insert(
                0,
                parse_quote! {
                    let __anodized_loop_maintains = #maintains_block;
                },
            );

            let let_decreases: Option<Stmt> = spec.decreases.as_ref().map(|loop_variant| {
                let expr = loop_variant.expr.clone();
                parse_quote! {
                    let _ = || #expr;
                }
            });
            stmts.insert(
                1,
                parse_quote! {
                    let __anodized_loop_decreases = {
                        #let_decreases
                    };
                },
            );
        }
    }
}

struct LoopSpecVisitor<'a> {
    config: &'a Config,
    errors: Option<Error>,
}

impl<'a> LoopSpecVisitor<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            errors: None,
        }
    }

    fn finish(self) -> Result<()> {
        match self.errors {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    fn add_error(&mut self, error: Error) {
        match &mut self.errors {
            Some(existing) => existing.combine(error),
            None => self.errors = Some(error),
        }
    }
}

impl VisitMut for LoopSpecVisitor<'_> {
    fn visit_expr_while_mut(&mut self, expr_while: &mut ExprWhile) {
        let attrs = std::mem::take(&mut expr_while.attrs);
        let (spec_attr, other_attrs) = match find_spec_attr(attrs) {
            Ok(result) => result,
            Err(error) => {
                self.add_error(error);
                return;
            }
        };
        expr_while.attrs = other_attrs;

        visit_mut::visit_expr_while_mut(self, expr_while);

        let Some(spec_attr) = spec_attr else {
            return;
        };

        match spec_attr.parse_args::<LoopSpec>() {
            Ok(spec) => self
                .config
                .instrument_loop_body(&spec, &mut expr_while.body.stmts),
            Err(error) => self.add_error(error),
        }
    }

    fn visit_expr_for_loop_mut(&mut self, expr_for_loop: &mut ExprForLoop) {
        let attrs = std::mem::take(&mut expr_for_loop.attrs);
        let (spec_attr, other_attrs) = match find_spec_attr(attrs) {
            Ok(result) => result,
            Err(error) => {
                self.add_error(error);
                return;
            }
        };
        expr_for_loop.attrs = other_attrs;

        visit_mut::visit_expr_for_loop_mut(self, expr_for_loop);

        let Some(spec_attr) = spec_attr else {
            return;
        };

        match spec_attr.parse_args::<LoopSpec>() {
            Ok(spec) => self.config.instrument_expr_for_loop(&spec, expr_for_loop),
            Err(error) => self.add_error(error),
        }
    }

    // Nested closure scopes are independently analyzed by the outer function macro expansion.
    fn visit_expr_closure_mut(&mut self, _expr_closure: &mut ExprClosure) {}

    // Nested `fn` items are independently analyzed by the outer function macro expansion.
    fn visit_item_fn_mut(&mut self, _item_fn: &mut ItemFn) {}
}
