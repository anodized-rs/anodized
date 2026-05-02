use proc_macro2::TokenStream;
use syn::{ExprWhile, Result};

use crate::{LoopSpec, instrument::Backend};

#[cfg(test)]
mod tests;

impl Backend {
    pub fn instrument_expr_while(
        &self,
        spec: LoopSpec,
        expr_while: &mut ExprWhile,
    ) -> Result<TokenStream> {
        let _ = spec;
        let _ = expr_while;
        todo!()
    }
}
