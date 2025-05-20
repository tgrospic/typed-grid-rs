use syn::parse::{Parse, ParseStream};
use syn::*;

/// Input parser: cols, rows
pub struct GridArgs {
    pub cols: usize,
    pub rows: usize,
}

impl Parse for GridArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let cols_lit: LitInt = input.parse()?;
        let cols: usize = cols_lit.base10_parse()?;

        if cols < 1 {
            return Err(Error::new(cols_lit.span(), "column must be at least 1"));
        }

        input.parse::<Token![,]>()?;
        let rows_lit: LitInt = input.parse()?;
        let rows: usize = rows_lit.base10_parse()?;

        if rows < 1 {
            return Err(Error::new(rows_lit.span(), "rows must be at least 1"));
        }

        Ok(GridArgs { cols, rows })
    }
}
