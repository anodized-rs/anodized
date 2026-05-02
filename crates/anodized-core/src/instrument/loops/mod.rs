use syn::{ExprWhile, Stmt, parse_quote};

use crate::{LoopSpec, instrument::Backend};

#[cfg(test)]
mod tests;

impl Backend {
    pub fn instrument_expr_while(&self, spec: LoopSpec, mut expr_while: ExprWhile) -> ExprWhile {
        self.instrument_loop_body(spec, &mut expr_while.body.stmts);
        expr_while
    }

    fn instrument_loop_body(&self, spec: LoopSpec, stmts: &mut Vec<Stmt>) {
        let maintains_block = Self::build_precondition_fn_body(&spec.maintains);
        stmts.insert(
            0,
            parse_quote! {
                let __anodized_loop_maintains = #maintains_block;
            },
        );

        let let_decreases: Option<Stmt> = spec.decreases.map(|loop_variant| {
            let expr = loop_variant.expr;
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
