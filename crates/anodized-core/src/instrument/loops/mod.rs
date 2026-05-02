use proc_macro2::TokenStream;
use syn::{ExprWhile, Result, parse_quote};

use crate::{LoopSpec, instrument::Backend};

#[cfg(test)]
mod tests;

impl Backend {
    pub fn instrument_expr_while(
        &self,
        spec: LoopSpec,
        expr_while: &mut ExprWhile,
    ) -> Result<TokenStream> {
        todo!()
    }
}
